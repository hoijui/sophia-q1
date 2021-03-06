extern crate sophia;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::rc::Rc;

use sophia::graph::{*, inmem::FastGraph};
use sophia::ns::Namespace;
use sophia::parser::turtle;
use sophia::serializer::*;
use sophia::serializer::nt::NtSerializer;
use sophia::triple::stream::TripleSource;
use sophia::triple::Triple;
use sophia::triple::streaming_mode::StreamedTriple;
use sophia::term::TTerm;
use sophia::term::Term;
use sophia::term::TermKind;

fn render_graph(graph : &FastGraph) {

    let mut indivs = HashSet::new();
    let mut literals = HashSet::new();
    let mut preds = HashSet::new();

    for triple in graph.triples() {
        triple.unwrap().s().bla();
        let &subj = (triple.expect("ff").s());
        let &pred = (triple.expect("ff").p());
        let &obj = (triple.expect("ff").o());

        indivs.insert(subj);

        preds.insert(pred);

        match obj.kind() {
            TermKind::Iri => { indivs.insert(obj); },
            TermKind::Literal => { literals.insert(obj); },
            TermKind::BlankNode => { },
            TermKind::Variable => { },
        };
    }
    // ...
}

fn main() {
    let data1 = "@base <http://example.org/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .

<http://www.w3.org/2001/sw/RDFCore/ntriples/> rdf:type foaf:Document ;
    <http://purl.org/dc/terms/title> \"N-Triples\"@en-US ;
    foaf:maker _:art .";
    let data2 = "<http://www.w3.org/2001/sw/RDFCore/ntriples/> <http://xmlns.com/foaf/0.1/maker> _:art .
             _:art <http://xmlns.com/foaf/0.1/name> \"Art Barstow\" .";
    let data = data2;

    let mut graph: FastGraph = turtle::parse_str(data).collect_triples().unwrap();

    render_graph(&graph);
}
