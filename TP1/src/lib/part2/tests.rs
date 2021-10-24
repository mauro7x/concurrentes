use actix::{Actor, Context, Handler, Message};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Mensajito {
    pub id: usize,
}

impl Handler<Mensajito> for MyActor {
    type Result = ();

    fn handle(&mut self, msg: Mensajito, _: &mut Context<Self>) {
        // OJO CON BLOQUEAR EL MAIN LOOP
        let id = msg.id;
        println!("Hola, soy {} me pasaron un id: {}", self.name, id);
    }
}

pub struct MyActor {
    pub name: String,
}

impl Default for MyActor {
    fn default() -> Self {
        MyActor {
            name: String::from("Santi"),
        }
    }
}

impl Actor for MyActor {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("[{}] I'm alive!!! Ready to rock!", self.name);
    }
}
