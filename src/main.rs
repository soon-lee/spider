mod handlers;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    // let app = routes::routes();
    //
    // let listener = tokio::net::TcpListener::bind(format!("{}:{}", env!("AXUM_HOST", "0.0.0.0"), env!("AXUM_PORT", "8000"))).await.unwrap();
    // axum::serve(listener, app).await.unwrap();


    // let playwright = playwright::Playwright::initialize().await.unwrap();
    // playwright.prepare().unwrap(); // Install browsers
    // let chromium = playwright.chromium();
    // let browser = chromium.launcher().headless(true).launch().await.unwrap();
    // let context = browser.context_builder().build().await.unwrap();
    // let page = context.new_page().await.unwrap();
    //
    // // 导航至百度首页
    // page.goto_builder("https://www.baidu.com").goto().await.unwrap();
    //
    // // 填写登录表单（这里省略了具体的登录操作，因为百度登录涉及到验证码等复杂流程）
    // // 通常，你需要定位用户名、密码输入框以及登录按钮，然后调用 `page.fill`, `page.click` 等方法
    //
    // // 搜索框定位并输入“rust 入门教程”
    // page.fill_builder("#kw", "rust 入门教程").fill().await.unwrap();
    //
    // // 点击搜索按钮
    // page.click_builder("#su").click().await.unwrap();
    //
    // // 打印页面标题，确认是否跳转到了搜索结果页面
    // println!("Page title is {}", page.title().await.unwrap());
    //
    // // 清理资源
    // browser.close().await.unwrap();

    println!("{}",crate::utils::crypt::auth_path("/api/user/getUserInfo".parse().unwrap()));
}
