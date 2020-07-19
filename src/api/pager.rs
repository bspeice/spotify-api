use crate::api::SpotifyClient;
use crate::model::page::Page;
use crate::Result;
use futures::future::BoxFuture;
use futures::ready;
use futures::stream::Stream;
use http_types::{Method, Request, Url};
use serde::de::DeserializeOwned;
use std::pin::Pin;
use std::task::{Context, Poll};

// TODO: Turn `BodyFuture` into a first-class future "DeserializeBody" future
type BodyFuture<'a> = BoxFuture<'a, Result<Vec<u8>>>;

fn poll_next<'a, T>(
    cx: &mut Context<'_>,
    client: &'a impl SpotifyClient,
    req: &mut Option<BodyFuture<'a>>,
    items: &mut Vec<T>,
    next: &mut Option<Url>,
) -> Poll<Option<Result<T>>>
where
    T: DeserializeOwned,
{
    loop {
        // If we have a request in progress, check to see if it's complete
        if let Some(ref mut f) = req {
            let resp = ready!(f.as_mut().poll(cx))?;

            // Request has completed, attempt to parse out the page
            let mut page = serde_json::from_slice::<Page<T>>(&resp)?;

            // Save the `next` URL for future use. This captures the `limit` and `offset`
            // params for us, so no worries about remembering those.
            if let Some(n) = page.next {
                next.replace(Url::parse(&n)?);
            }

            // Queue all items, and fall through to returning them individually
            for item in page.items.drain(..) {
                items.push(item);
            }
        }

        // Check if we have a buffered item
        if !items.is_empty() {
            return Poll::Ready(Some(Ok(items.remove(0))));
        }

        if let Some(next) = next.take() {
            // If we're out of buffered items, start the next request
            let next_req = Request::new(Method::Get, next);
            let f = Box::pin(async move {
                let mut resp = client.send_authorized(next_req).await?;
                Ok(resp.body_bytes().await?)
            });
            req.replace(f);
        } else {
            // Otherwise, we can't make a request, we're done
            return Poll::Ready(None);
        }
    }
}

pub struct Pager<'a, C, T> {
    client: &'a C,
    req: Option<BodyFuture<'a>>,
    items: Vec<T>,
    next: Option<Url>,
}

impl<'a, C, T> Pager<'a, C, T> {
    pub(crate) fn new(client: &'a C, next: Option<Url>) -> Self {

        // TODO: Smarter capacity hint
        Self {
            client,
            req: None,
            items: Vec::with_capacity(50),
            next: next,
        }
    }

    pub(crate) fn with_items(client: &'a C, items: Vec<T>, next: Option<Url>) -> Self {
        Self {
            client,
            req: None,
            items,
            next
        }
    }
}

impl<'a, C, T> Stream for Pager<'a, C, T>
where
    C: SpotifyClient,
    T: DeserializeOwned + Unpin,
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
