use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iri_string::format::ToDedicatedString;
use oxiri::{Iri as Oxiri, IriRef as OxiriRef};

fn abs_examples() -> impl Iterator<Item=&'static str> {
    [
        "file://foo",
        "ftp://ftp.is.co.za/rfc/rfc1808.txt",
        "http://www.ietf.org/rfc/rfc2396.txt",
        //"ldap://[2001:db8::7]/c=GB?objectClass?one",
        "mailto:John.Doe@example.com",
        "news:comp.infosystems.www.servers.unix",
        "tel:+1-816-555-1212",
        "telnet://192.0.2.16:80/",
        "urn:oasis:names:specification:docbook:dtd:xml:4.1.2",
        "http://example.com",
        "http://example.com/",
        "http://example.com/foo",
        "http://example.com/foo/bar",
        "http://example.com/foo/bar/",
        "http://example.com/foo/bar?q=1&r=2",
        "http://example.com/foo/bar/?q=1&r=2",
        "http://example.com#toto",
        "http://example.com/#toto",
        "http://example.com/foo#toto",
        "http://example.com/foo/bar#toto",
        "http://example.com/foo/bar/#toto",
        "http://example.com/foo/bar?q=1&r=2#toto",
        "http://example.com/foo/bar/?q=1&r=2#toto",
        "http://example.com/foo/bar/.././baz",
    ].iter().copied()
}

fn rel_examples() -> impl Iterator<Item=&'static str> {
    [
        "g:h",
        "g",
        "g/",
        "/g",
        "//g",
        "?y",
        "g?y",
        "#s",
        "g#s",
        "g?y#s",
        ";x",
        "g;x",
        "g;x?y#s",
        "",
        ".",
        "./",
        "./g",
        "..",
        "../",
        "../g",
        "../..",
        "../../",
        "../../g",
        "../../../g",
        "../../../../g",
        "/./g",
        "/../g",
        "g.",
        ".g",
        "g..",
        "..g",
        "./../g",
        "./g/.",
        "g/./h",
        "g/../h",
        "g;x=1/./y",
        "g;x=1/../y",
        "g?y/./x",
        "g?y/../x",
        "g#s/./x",
        "g#s/../x",
        "http:g",
        "./g:h",
    ].iter().copied()
}

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse IRI");
    group.bench_function("iri-string", |b| {
        b.iter(|| {
            for iri in abs_examples() {
                black_box(iri_string::types::IriStr::new(iri).unwrap());
            }
        })
    });
    group.bench_function("iref", |b| {
        b.iter(|| {
            for iri in abs_examples() {
                black_box(iref::Iri::new(iri).unwrap());
            }
        })
    });
    group.bench_function("oxiri", |b| {
        b.iter(|| {
            for iri in abs_examples() {
                black_box(Oxiri::parse(iri).unwrap());
            }
        })
    });
    group.bench_function("sophia_iri", |b| {
        b.iter(|| {
            for iri in abs_examples() {
                black_box(sophia_iri::Iri::new(iri).unwrap());
            }
        })
    });
    group.finish();
}

fn bench_parse_ref_abs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse IRI reference absolute");
    group.bench_function("iri-string", |b| {
        b.iter(|| {
            for iriref in abs_examples() {
                black_box(iri_string::types::IriReferenceStr::new(iriref).unwrap());
            }
        })
    });
    group.bench_function("iref", |b| {
        b.iter(|| {
            for iriref in abs_examples() {
                black_box(iref::IriRef::new(iriref).unwrap());
            }
        })
    });
    group.bench_function("oxiri", |b| {
        b.iter(|| {
            for iriref in abs_examples() {
                black_box(OxiriRef::parse(iriref).unwrap());
            }
        })
    });
    group.bench_function("sophia_iri", |b| {
        b.iter(|| {
            for iriref in abs_examples() {
                black_box(sophia_iri::IriRef::new(iriref).unwrap());
            }
        })
    });
    group.finish();
}

fn bench_parse_ref_rel(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse IRI reference relative");
    group.bench_function("iri-string", |b| {
        b.iter(|| {
            for iriref in rel_examples() {
                black_box(iri_string::types::IriReferenceStr::new(iriref).unwrap());
            }
        })
    });
    group.bench_function("iref", |b| {
        b.iter(|| {
            for iriref in rel_examples() {
                black_box(iref::IriRef::new(iriref).unwrap());
            }
        })
    });
    group.bench_function("oxiri", |b| {
        b.iter(|| {
            for iriref in rel_examples() {
                black_box(OxiriRef::parse(iriref).unwrap());
            }
        })
    });
    group.bench_function("sophia_iri", |b| {
        b.iter(|| {
            for iriref in rel_examples() {
                black_box(sophia_iri::IriRef::new(iriref).unwrap());
            }
        })
    });
    group.finish();
}

fn bench_resolve(c: &mut Criterion) {
    static BASE: &str = "http://a/b/c/d;p?q";

    let mut group = c.benchmark_group("Resolve relative IRI reference against a fixed base IRI");

    group.bench_function("iri-string", |b| {
        let base = iri_string::types::IriAbsoluteStr::new(BASE).unwrap();
        let resolver = iri_string::resolve::FixedBaseResolver::new(base);
        b.iter(|| {
            for relative in rel_examples() {
                black_box(resolver.resolve(iri_string::types::IriReferenceStr::new(relative).unwrap()).to_dedicated_string());
            }
        })
    });
    group.bench_function("iref", |b| {
        let base = iref::Iri::new(BASE).unwrap();
        b.iter(|| {
            for relative in rel_examples() {
                let mut rel = iref::IriRefBuf::new(relative.to_string()).unwrap();
                black_box(rel.resolve(base));
            }
        })
    });
    group.bench_function("oxiri", |b| {
        let base = Oxiri::parse(BASE).unwrap();
        b.iter(|| {
            for relative in rel_examples() {
                let mut buf = String::new();
                black_box(base.resolve_into(relative, &mut buf).unwrap());
            }
        })
    });
    group.bench_function("sophia_iri", |b| {
        let base = sophia_iri::resolve::BaseIri::new(BASE).unwrap();
        b.iter(|| {
            for relative in rel_examples() {
                let mut buf = String::new();
                black_box(base.resolve_into(relative, &mut buf).unwrap());
            }
        })
    });
    group.bench_function("oxiri Mbuf", |b| {
        let base = Oxiri::parse(BASE).unwrap();
        let mut buf = String::new();
        b.iter(|| {
            for relative in rel_examples() {
                buf.clear();
                black_box(base.resolve_into(relative, &mut buf).unwrap());
            }
        })
    });
    group.bench_function("sophia_iri Mbuf", |b| {
        let base = sophia_iri::resolve::BaseIri::new(BASE).unwrap();
        let mut buf = String::new();
        b.iter(|| {
            for relative in rel_examples() {
                buf.clear();
                black_box(base.resolve_into(relative, &mut buf).unwrap());
            }
        })
    });
    group.finish();
}

criterion_group!(iri, 
    bench_parse,
    bench_parse_ref_abs,
    bench_parse_ref_rel,
    bench_resolve,
);
criterion_main!(iri);
