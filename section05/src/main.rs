// use anyhow::Result;
// // use axum::Router;
// // use section05::app_state::{get_router, AppState};
// // use tokio::net::TcpListener;
// use tracing::{info, level_filters::LevelFilter};
// use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};
// // use section05::config::AppConfig;
// // use section05::error::AppError;
// use gpui::*;

// fn main() {
//     App::new().run(|cx: &mut AppContext| {
//         Window::new("win", cx)
//             .title("我的第一个 GPUI 窗口")
//             .build(cx, |cx| Label::new("Hello，GPUI！", cx))
//             .unwrap();
//     });
// }

// // #[tokio::main]
// // async fn main() -> Result<()> {
// //     println!("section05_chat_room!");

// //     // App::new().run(|cx: &mut AppContext| {
// //     //     Window::new("win", cx)
// //     //         .title("我的第一个 GPUI 窗口")
// //     //         .build(cx, |cx| Label::new("Hello，GPUI！", cx))
// //     //         .unwrap();
// //     // });

// //     // 日志收集
// //     // let layer = Layer::new().with_filter(LevelFilter::INFO);
// //     // tracing_subscriber::registry().with(layer).init();

// //     // // 加载配置
// //     // let config: AppConfig = AppConfig::load()?;

// //     // let port: u16 = config.server.port;
// //     // let addr: String = format!("0.0.0.0:{}", port); // 拼接url地址

// //     // let state: AppState = AppState::new(config).await;
// //     // let app: Router = get_router(state); // state用于with_state(state)

// //     // // 创建tcp并绑定到addr地址(异步操作)
// //     // let listener: TcpListener = TcpListener::bind(&addr).await?;
// //     // info!("服务启动: {}", addr);

// //     // // 根据tcp实例, 创建服务
// //     // axum::serve(listener, app.into_make_service()).await?;
// //     Ok(())
// // }


use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::red())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::green())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::blue())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::yellow())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::black())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .rounded_md()
                            .border_color(gpui::white()),
                    )
                    .child(
                        div()
                            .size_8()
                            .bg(gpui::white())
                            .border_1()
                            .border_dashed()
                            .rounded_md()
                            .border_color(gpui::black()),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}