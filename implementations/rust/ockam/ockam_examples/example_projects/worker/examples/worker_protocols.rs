use ockam::{
    async_worker,
    protocols::{stream::responses::*, ProtocolParser},
    Any, Context, Result, Routed, Worker,
};

#[derive(Default)]
struct MyWorker {
    parser: ProtocolParser<MyWorker>,
    stream: Option<String>,
}

#[async_worker]
impl Worker for MyWorker {
    type Context = Context;
    type Message = Any;

    async fn initialize(&mut self, _: &mut Context) -> Result<()> {
        self.parser
            .attach(ResponseParser::new(|worker: &mut Self, msg| match msg {
                Response::Init(Init { stream_name }) => worker.stream = Some(stream_name),
                _ => { /* ignore messages */ }
            }));

        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: Routed<Any>) -> Result<()> {
        self.parser.prepare().parse(self, msg)?;

        println!("Stream name is now: `{:?}`", self.stream);
        ctx.stop().await
    }
}

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    ctx.start_worker("worker", MyWorker::default()).await?;

    // Send an Init message to our worker -- this message would
    // normally be sent from the Ockam Hub stream service
    ctx.send("worker", Init::new("test-stream")).await?;

    Ok(())
}
