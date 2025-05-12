use rust_on_rails::prelude::*;
use pelican_ui::prelude::*;

mod screens;
use screens::*;

pub struct MyApp;

impl App for MyApp {
    // async fn background_tasks(ctx: &mut HeadlessContext) -> Tasks {
    //    // BDKPlugin::background_tasks(ctx).await
    // }
    async fn plugins(ctx: &mut Context, h_ctx: &mut HeadlessContext) -> (Plugins, Tasks) {
        let (pelican, tasks) = PelicanUI::new(ctx, h_ctx).await;
        
        (std::collections::HashMap::from([
            (std::any::TypeId::of::<PelicanUI>(), Box::new(pelican) as Box<dyn std::any::Any>)
        ]), tasks)
    }

    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        let home = SelectInstitution::new(ctx);
        Box::new(Interface::new(ctx, home, None, None))
    }
}

create_entry_points!(MyApp);