use juniper;
use juniper::EmptyMutation;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use iron::Request;
use library::GlobalLibrary;
use mount::Mount;

struct Context {
    //library: GlobalLibrary
}

impl juniper::Context for Context {}

struct Query;

graphql_object!(Query: Context |&self| {

});

struct Mutation;

graphql_object!(Mutation: Context |&self| {
});

fn context_factory(_: &mut Request) -> Context {
    Context {}
}

pub fn build(library: GlobalLibrary) -> Mount {//(GraphQLHandler<Fn(&mut Request) -> Context, Context, Query, EmptyMutation<()>>, GraphiQLHandler) {
    let mut mount = Mount::new();
    let graphql = GraphQLHandler::new(
        context_factory,
        Query,
        Mutation,
    );
    let graphiql = GraphiQLHandler::new("/graphql");
    mount.mount("/graphql", graphql);
    mount.mount("/graphiql", graphiql);

    mount
}