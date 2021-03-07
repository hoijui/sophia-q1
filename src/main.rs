extern crate sophia;

use std::collections::HashSet;
use std::error::Error;

use sophia::graph::{inmem::FastGraph, *};
use sophia::parser::turtle;
use sophia::term::TermKind;
use sophia::term::{BoxTerm, CopiableTerm, TTerm};
use sophia::triple::stream::TripleSource;
use sophia::triple::Triple;

fn render_graph(graph: &FastGraph) -> Result<(), Box<dyn Error>> {
    let mut indivs = HashSet::new();
    let mut literals = HashSet::new();
    let mut preds = HashSet::new();

    for triple in graph.triples() {
        let triple = triple?;

        // triple.s() is a reference, lifing only as long as `triple`
        // i.e. until the end of the iteration,
        // i.e. not long enough to be safely stored in `indivs`
        // (which will still exist after the end of the loop).
        //
        // Therefore, we must make a self-contained copy of the term.
        // By self-contained, I mean a term that we own (so that we can move it into `indivs`)
        // and that owns its own underlying data.
        // The simplest such type of term is BoxTerm.
        //
        // The `copied()` method of terms (defined in the trait `CopiableTerm`)
        // is generic over its return type, much like `into()`.

        let subj: BoxTerm = triple.s().copied();
        indivs.insert(subj);

        // Let's do the same for predicate and object.

        let pred: BoxTerm = triple.p().copied();
        preds.insert(pred);

        let obj: BoxTerm = triple.o().copied();
        match obj.kind() {
            TermKind::Iri => {
                indivs.insert(obj);
            }
            TermKind::Literal => {
                literals.insert(obj);
            }
            TermKind::BlankNode => {}
            TermKind::Variable => {}
        };
    }
    println!(
        "{} individuals, {} predicates, {} literals",
        indivs.len(),
        preds.len(),
        literals.len()
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    #[allow(unused_variables)]
    let data1 = "@base <http://example.org/> .
        @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        @prefix foaf: <http://xmlns.com/foaf/0.1/> .

        <http://www.w3.org/2001/sw/RDFCore/ntriples/> rdf:type foaf:Document ;
            <http://purl.org/dc/terms/title> \"N-Triples\"@en-US ;
            foaf:maker _:art .";

    #[allow(unused_variables)]
    let data2 =
        "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
             _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";
    let data = data2;

    let graph: FastGraph = turtle::parse_str(data).collect_triples().unwrap();

    render_graph(&graph)
}
