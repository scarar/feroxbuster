use super::*;
use ::fuzzyhash::FuzzyHash;
use ::regex::Regex;


#[test]
/// just a simple test to increase code coverage by hitting as_any and the inner value
fn lines_filter_as_any() {
    let filter = LinesFilter { line_count: 1 };
    let filter2 = LinesFilter { line_count: 1 };

    assert!(filter.box_eq(filter2.as_any()));

    assert_eq!(filter.line_count, 1);
    assert_eq!(
        *filter.as_any().downcast_ref::<LinesFilter>().unwrap(),
        filter
    );
}

#[test]
/// just a simple test to increase code coverage by hitting as_any and the inner value
fn words_filter_as_any() {
    let filter = WordsFilter { word_count: 1 };
    let filter2 = WordsFilter { word_count: 1 };

    assert!(filter.box_eq(filter2.as_any()));

    assert_eq!(filter.word_count, 1);
    assert_eq!(
        *filter.as_any().downcast_ref::<WordsFilter>().unwrap(),
        filter
    );
}

#[test]
/// just a simple test to increase code coverage by hitting as_any and the inner value
fn size_filter_as_any() {
    let filter = SizeFilter { content_length: 1 };
    let filter2 = SizeFilter { content_length: 1 };

    assert!(filter.box_eq(filter2.as_any()));

    assert_eq!(filter.content_length, 1);
    assert_eq!(
        *filter.as_any().downcast_ref::<SizeFilter>().unwrap(),
        filter
    );
}

#[test]
/// just a simple test to increase code coverage by hitting as_any and the inner value
fn status_code_filter_as_any() {
    let filter = StatusCodeFilter { filter_code: 200 };
    let filter2 = StatusCodeFilter { filter_code: 200 };

    assert!(filter.box_eq(filter2.as_any()));

    assert_eq!(filter.filter_code, 200);
    assert_eq!(
        *filter.as_any().downcast_ref::<StatusCodeFilter>().unwrap(),
        filter
    );
}

#[test]
/// just a simple test to increase code coverage by hitting as_any and the inner value
fn regex_filter_as_any() {
    let raw = r".*\.txt$";
    let compiled = Regex::new(raw).unwrap();
    let compiled2 = Regex::new(raw).unwrap();
    let filter = RegexFilter {
        compiled,
        raw_string: raw.to_string(),
    };
    let filter2 = RegexFilter {
        compiled: compiled2,
        raw_string: raw.to_string(),
    };

    assert!(filter.box_eq(filter2.as_any()));

    assert_eq!(filter.raw_string, r".*\.txt$");
    assert_eq!(
        *filter.as_any().downcast_ref::<RegexFilter>().unwrap(),
        filter
    );
}

#[test]
/// test should_filter on RegexFilter where regex matches body
fn regexfilter_should_filter_when_regex_matches_on_response_body() {
    let mut resp = FeroxResponse::default();
    resp.set_url("http://localhost/stuff");
    resp.set_text("im a body response hurr durr!");

    let raw = r"response...rr";

    let filter = RegexFilter {
        raw_string: raw.to_string(),
        compiled: Regex::new(raw).unwrap(),
    };

    assert!(filter.should_filter_response(&resp));
}

#[test]
/// a few simple tests for similarity filter
fn similarity_filter_is_accurate() {
    let mut resp = FeroxResponse::default();
    resp.set_url("http://localhost/stuff");
    resp.set_text("sitting");

    let mut filter = SimilarityFilter {
        hash: HashValueType::String(FuzzyHash::new("kitten").to_string()),
        threshold: 95,
        original_url: "".to_string(),
    };

    // kitten/sitting is 57% similar, so a threshold of 95 should not be filtered
    assert!(!filter.should_filter_response(&resp));

    resp.set_text("");
    filter.hash = HashValueType::String(String::new());
    filter.threshold = 100;

    // two empty strings are the same, however ssdeep doesn't accept empty strings, expect false
    assert!(!filter.should_filter_response(&resp));

    resp.set_text("some data to hash for the purposes of running a test");
    filter.hash = HashValueType::String(FuzzyHash::new("some data to hash for the purposes of running a te").to_string());
    filter.threshold = 17;

    assert!(filter.should_filter_response(&resp));
}

#[test]
/// just a simple test to increase code coverage by hitting as_any and the inner value
fn similarity_filter_as_any() {
    let filter = SimilarityFilter {
        hash: HashValueType::String(String::from("stuff")),
        threshold: 95,
        original_url: "".to_string(),
    };

    let filter2 = SimilarityFilter {
        hash: HashValueType::String(String::from("stuff")),
        threshold: 95,
        original_url: "".to_string(),
    };

    assert!(filter.box_eq(filter2.as_any()));

    assert_eq!(
        *filter.as_any().downcast_ref::<SimilarityFilter>().unwrap(),
        filter
    );
}

#[test]
/// test correctness of FeroxFilters::remove
fn remove_function_works_as_expected() {
    let data = FeroxFilters::default();
    assert!(data.filters.read().unwrap().is_empty());

    (0..8).for_each(|i| {
        data.push(Box::new(WordsFilter { word_count: i })).unwrap();
    });

    // remove removes index-1 from the vec, zero is skipped, and out-of-bounds indices are skipped
    data.remove(&mut [0]);
    assert_eq!(data.filters.read().unwrap().len(), 8);

    data.remove(&mut [10000]);
    assert_eq!(data.filters.read().unwrap().len(), 8);

    // removing 0, 2, 4
    data.remove(&mut [1, 3, 5]);

    assert_eq!(data.filters.read().unwrap().len(), 5);

    let expected = vec![
        WordsFilter { word_count: 1 },
        WordsFilter { word_count: 3 },
        WordsFilter { word_count: 5 },
        WordsFilter { word_count: 6 },
        WordsFilter { word_count: 7 },
    ];

    for filter in data.filters.read().unwrap().iter() {
        let downcast = filter.as_any().downcast_ref::<WordsFilter>().unwrap();
        assert!(expected.contains(downcast));
    }
}
