use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, env, fs};

// 定义Handler trait，所有请求处理器都需要实现这个trait
// Define Handler trait that all request handlers must implement
pub trait Handler {
    // 处理请求并返回响应
    // Handle the request and return a response
    fn handle(req: &HttpRequest) -> HttpResponse;

    // 从文件系统加载文件内容的辅助方法
    // Helper method to load file contents from filesystem
    fn load_file(file_name: &str) -> Option<String> {
        // 获取默认的public目录路径（基于项目根目录）
        // Get default public directory path (based on project root)
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        
        // 尝试从环境变量获取PUBLIC_PATH，如果没有则使用默认路径
        // Try to get PUBLIC_PATH from environment variable, use default if not found
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        
        // 构建完整的文件路径
        // Build the complete file path
        let full_path = format!("{}/{}", public_path, file_name);

        // 尝试读取文件内容
        // Try to read file contents
        let contents = fs::read_to_string(full_path);
        
        // 如果读取成功返回Some(内容)，失败返回None
        // Return Some(contents) if successful, None if failed
        contents.ok()
    }
}

// 订单状态结构体，用于JSON序列化/反序列化
// Order status struct for JSON serialization/deserialization
#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,      // 订单ID
    order_date: String, // 订单日期
    order_status: String, // 订单状态
}

// 静态页面处理器
// Static page handler
pub struct StaticPageHandler;

impl Handler for StaticPageHandler {
    // 处理静态页面请求
    // Handle static page requests
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resuource::Path(s) = &req.resource;

        // 将路径按'/'分割成路由片段
        // Split path into route segments by '/'
        let route: Vec<&str> = s.split('/').collect();
        match route.get(1) {
            // 根路径返回首页
            // Root path returns home page
            Some(&"") => HttpResponse::new("200", None, Self::load_file("index.html")),
            // 健康检查页面
            // Health check page
            Some(&"health") => HttpResponse::new("200", None, Self::load_file("health.html")),
            // 其他路径尝试加载对应文件
            // Try to load corresponding file for other paths
            Some(path) => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    // 根据文件扩展名设置Content-Type
                    // Set Content-Type based on file extension
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                // 文件不存在返回404
                // Return 404 if file doesn't exist
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
            // 路径为空返回404
            // Return 404 if path is empty
            None => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}

// 404页面处理器
// 404 page handler
pub struct PageNotFoundHandler;

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        // 返回404页面未找到响应
        // Return 404 Page Not Found response
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

// Web服务处理器，用于处理API请求
// Web service handler for API requests
pub struct WebServiceHandler;

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resuource::Path(s) = &req.resource;
        // 将路径按'/'分割成路由片段
        // Split path into route segments by '/'
        let route: Vec<&str> = s.split('/').collect();
        
        // 匹配API路由
        // Match API routes
        match route.get(2) {
            // 处理 /api/shipping/orders 路径
            // Handle /api/shipping/orders path
            Some(&"shipping") if route.len() > 3 && route.get(3) == Some(&"orders") => {
                // 序列化订单数据为JSON
                // Serialize order data to JSON
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            // 其他路径返回404
            // Return 404 for other paths
            _ => {
                HttpResponse::new("404", None, Self::load_file("404.html"))
            }
        }
    }
}

impl WebServiceHandler {
    // 加载模拟订单JSON数据的方法
    // Method to load mock order JSON data
    fn load_json() -> Vec<OrderStatus> {
        vec![
            OrderStatus {
                order_id: 1,
                order_date: "2024-06-01".to_string(),
                order_status: "Shipped".to_string(), // 已发货
            },
            OrderStatus {
                order_id: 2,
                order_date: "2024-06-02".to_string(),
                order_status: "Processing".to_string(), // 处理中
            },
            OrderStatus {
                order_id: 3,
                order_date: "2024-06-03".to_string(),
                order_status: "Delivered".to_string(), // 已送达
            },
        ]
    }
}