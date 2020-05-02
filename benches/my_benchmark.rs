use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oxiri::{Iri as Oxiri, IriRef as OxiriRef};
use sophia_iri::*;

fn abs_examples() -> &'static [&'static str] {
    &[
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
    ]
}

fn bench_oxiri_parse(c: &mut Criterion) {
    c.bench_function("parse abs oxiri", |b| {
        b.iter(|| {
            for iri in abs_examples().iter() {
                black_box(Oxiri::parse(*iri).unwrap());
            }
        })
    });
}

fn bench_oxiri_parse_relative(c: &mut Criterion) {
    c.bench_function("parse rel oxiri", |b| {
        b.iter(|| {
            for iri in abs_examples().iter() {
                black_box(OxiriRef::parse(*iri).unwrap());
            }
        })
    });
}

fn bench_sophia_iri_parse(c: &mut Criterion) {
    c.bench_function("parse abs sophia", |b| {
        b.iter(|| {
            for iri in abs_examples().iter() {
                black_box(Iri::new(*iri).unwrap());
            }
        })
    });
}

fn bench_sophia_iri_parse_relative(c: &mut Criterion) {
    c.bench_function("parse rel sophia", |b| {
        b.iter(|| {
            for iri in abs_examples().iter() {
                black_box(IriRef::new(*iri).unwrap());
            }
        })
    });
}

fn bench_iref_parse(c: &mut Criterion) {
    c.bench_function("parse abs iref", |b| {
        b.iter(|| {
            for iri in abs_examples().iter() {
                black_box(iref::Iri::new(*iri).unwrap());
            }
        })
    });
}

fn bench_iref_parse_relative(c: &mut Criterion) {
    c.bench_function("parse rel iref", |b| {
        b.iter(|| {
            for iri in abs_examples().iter() {
                black_box(iref::IriRef::new(*iri).unwrap());
            }
        })
    });
}

fn rel_examples() -> &'static [&'static str] {
    &[
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
    ]
}

fn bench_oxiri_resolve(c: &mut Criterion) {
    let base = Oxiri::parse("http://a/b/c/d;p?q").unwrap();

    let mut buf = String::new();
    c.bench_function("resolve oxiri", |b| {
        b.iter(|| {
            for relative in rel_examples().iter() {
                buf.clear();
                black_box(base.resolve_into(relative, &mut buf).unwrap());
            }
        })
    });
}

fn bench_sophia_resolve(c: &mut Criterion) {
    let base = resolve::BaseIri::new("http://a/b/c/d;p?q").unwrap();

    let mut buf = String::new();
    c.bench_function("resolve sophia", |b| {
        b.iter(|| {
            for relative in rel_examples().iter() {
                buf.clear();
                black_box(base.resolve_into(*relative, &mut buf).unwrap());
            }
        })
    });
}

// This bench is much slower than the ones for oxiri and sophia,
// because it allocates a new String on each test, while oxiri and sophia reuse the same buffer all along.
//
// This may be consider an unfair comparison, but then iref does not provide any way to reuse a buffer...
fn bench_iref_resolve(c: &mut Criterion) {
    let base = iref::Iri::new("http://a/b/c/d;p?q").unwrap();

    c.bench_function("resolve iref", |b| {
        b.iter(|| {
            for relative in rel_examples().iter() {
                let mut rel = iref::IriRefBuf::new(relative.to_string()).unwrap();
                black_box(rel.resolve(base));
            }
        })
    });
}

criterion_group!(iri, bench_oxiri_parse, bench_oxiri_parse_relative, bench_sophia_iri_parse, bench_sophia_iri_parse_relative, bench_iref_parse, bench_iref_parse_relative, bench_oxiri_resolve, bench_sophia_resolve, bench_iref_resolve);

criterion_main!(iri);
