use crate::api::client::{ClientExt, SpotifyClient};
use crate::Result;
use futures::future::BoxFuture;
use futures::ready;
use futures::stream::Stream;
use http_types::{Method, Request, Url};
use serde::de::DeserializeOwned;
use std::mem::swap;
use std::pin::Pin;
use std::task::{Context, Poll};

type BodyFuture<'a, T> = BoxFuture<'a, Result<T>>;

// NOTE: While `Pageable` isn't intended to be implemented by anything outside this crate,
// because `Pager` needs to be publically available, this does as well.
pub trait Pageable<T>: DeserializeOwned {
    fn next_url(&self) -> Option<&str>;
    fn into_items(self) -> Vec<T>;
}

fn poll_next<'a, C, T, P>(
    cx: &mut Context<'_>,
    client: &'a C,
    req: &mut Option<BodyFuture<'a, P>>,
    items: &mut Vec<T>,
    next: &mut Option<Url>,
) -> Poll<Option<Result<T>>>
where
    C: SpotifyClient + ?Sized,
    T: DeserializeOwned,
    P: Pageable<T>,
{
    loop {
        // If we have a request in progress, check to see if it's complete
        if let Some(ref mut f) = req {
            let page = ready!(f.as_mut().poll(cx))?;

            // If we've finished, drop the current request Future so we don't attempt to re-poll
            req.take();

            // Save the `next` URL for future use. This captures the `limit` and `offset`
            // params for us, so no worries about remembering those.
            if let Some(n) = page.next_url() {
                next.replace(Url::parse(n)?);
            }

            // Queue all items, and fall through to returning them individually
            swap(items, &mut page.into_items());
        }

        // Return the next item if there are any available
        if !items.is_empty() {
            return Poll::Ready(Some(Ok(items.remove(0))));
        }

        if let Some(next) = next.take() {
            // No items available, so start the next request and fall through to loop around and
            // poll it
            let next_req = Request::new(Method::Get, next);
            let f = client.send_authorized(next_req).deserialize_response::<P>();
            req.replace(f);
        } else {
            // No future requests to make, this stream has ended
            return Poll::Ready(None);
        }
    }
}

pub struct Pager<'a, C: ?Sized, T, P> {
    client: &'a C,
    req: Option<BodyFuture<'a, P>>,
    items: Vec<T>,
    next: Option<Url>,
}

impl<'a, C: ?Sized, T, P> Pager<'a, C, T, P> {
    pub(crate) fn with_items(client: &'a C, items: Vec<T>, next: Option<Url>) -> Self {
        Self {
            client,
            req: None,
            items,
            next,
        }
    }
}

impl<'a, C, T, P> Stream for Pager<'a, C, T, P>
where
    C: SpotifyClient + ?Sized,
    T: DeserializeOwned + Unpin,
    P: Pageable<T>,
{
    type Item = Result<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Pager {
            client,
            ref mut req,
            ref mut items,
            ref mut next,
        } = &mut *self;
        poll_next(cx, *client, req, items, next)
    }
}
