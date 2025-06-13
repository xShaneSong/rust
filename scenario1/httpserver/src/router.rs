use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => {
                match req.resource {
                    httprequest::Resuource::Path(path) => {
                        let route: Vec<&str> = path.split('/').collect();
                        match route[1] {
                            "api" => {
                                let resp: HttpResponse = WebServiceHandler::handle(&req);
                                let _ = resp.send_response(stream);
                            }
                            _ => {
                                let resp: HttpResponse = StaticPageHandler::handle(&req);
                                let _ = resp.send_response(stream);
                            }
                        }

                        // if path == "/" {
                        //     StaticPageHandler::handle(req, stream);
                        // } else if path.starts_with("/api/") {
                        //     WebServiceHandler::handle(req, stream);
                        // } else {
                        //     PageNotFoundHandler::handle(req, stream);
                        // }
                    }
                }
            }
            httprequest::Method::Post => {
                WebServiceHandler::handle(req, stream);
            }
            _ => {
                let response = PageNotFoundHandler::handle(&req);
                let _ = response.send_response(stream);
            }
        }

    }
}