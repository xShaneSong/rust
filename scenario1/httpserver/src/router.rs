use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

// 路由器结构体，负责根据请求类型分发到不同的处理器
// Router struct responsible for dispatching requests to different handlers
pub struct Router;

impl Router {
    // 路由方法：根据HTTP请求的方法和路径，决定使用哪个处理器
    // Route method: determines which handler to use based on HTTP request method and path
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        // 根据HTTP方法进行匹配
        // Match based on HTTP method
        match req.method {
            // 处理GET请求
            // Handle GET requests
            httprequest::Method::Get => {
                // 根据请求的资源路径进行匹配
                // Match based on the requested resource path
                match &req.resource {
                    httprequest::Resuource::Path(path) => {
                        // 将路径按'/'分割成路由段
                        // Split the path into route segments by '/'
                        let route: Vec<&str> = path.split('/').collect();
                        // 根据第一个路由段决定处理方式
                        // Determine handling based on the first route segment
                        match route[1] {
                            // 如果是API请求（/api/...）
                            // If it's an API request (/api/...)
                            "api" => {
                                // 使用WebService处理器处理API请求
                                // Use WebService handler for API requests
                                let resp: HttpResponse = WebServiceHandler::handle(&req);
                                // 发送响应到客户端
                                // Send response to client
                                let _ = resp.send_response(stream);
                            }
                            // 其他所有GET请求（静态页面）
                            // All other GET requests (static pages)
                            _ => {
                                // 使用静态页面处理器
                                // Use static page handler
                                let resp: HttpResponse = StaticPageHandler::handle(&req);
                                // 发送响应到客户端
                                // Send response to client
                                let _ = resp.send_response(stream);
                            }
                        }
                    }
                }
            }
            // 处理其他未支持的HTTP方法
            // Handle other unsupported HTTP methods
            _ => {
                // 返回404页面未找到响应
                // Return 404 Page Not Found response
                let response = PageNotFoundHandler::handle(&req);
                // 发送404响应到客户端
                // Send 404 response to client
                let _ = response.send_response(stream);
            }
        }
    }
}