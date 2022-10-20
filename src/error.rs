use rocket::{http::Status, response::Responder, Request};
use rocket_dyn_templates::{context, Template};

#[derive(Debug)]
pub struct ErrorPage(anyhow::Error);

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ErrorPage {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        warn!("{}", self.0);
        let error = self.0.to_string();
        let template = Template::render("error", context! { error });
        let mut res = template.respond_to(request)?;
        res.set_status(Status::InternalServerError);
        Ok(res)
    }
}

impl<T> From<T> for ErrorPage
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        ErrorPage(t.into())
    }
}
