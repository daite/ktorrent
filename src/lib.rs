pub use select::document::Document;
pub use select::predicate::{Class, Name};
pub use scrape::*;

/// # Example
/// ```
/// let bbs_doc = ktorrent::Document::from(include_str!("./test_data/torrentsir_bbs.html"));
/// let data = ktorrent::find_child_attr_by_tag(
///    &bbs_doc, "list-group", "a", "href")[1];
///  assert_eq!(
///     "magnet:?xt=urn:btih:27646d3df274ed51b6386bd6aa40da849a73b341",
///      data,
///  );
/// ```
pub mod scrape {
    use super::*;
    /// it collects a child attribute by a child tag name.
    /// <li class="p_class_name">
    ///     <i class="c_class_name"></i> 
    ///     <c_tag_name c_attr_name=c_attr_name>xxx</a>
    /// </li>
    pub fn find_child_attr_by_tag<'a> (
        doc: &'a Document, 
        p_class_name: &'a str, 
        c_tag_name: &'a str,
        c_attr_name: &'a str) -> Vec<&'a str> {
        let mut data = vec![];
        for node in doc.find(Class(p_class_name)) {
            let val = node.find(Name(c_tag_name))
                        .next()
                        .unwrap()
                        .attr(c_attr_name)
                        .unwrap();
            data.push(val);     
        }
        data
    }
    /// it collects a text from a parent node which has a class name of a child.
    /// <b>
    /// <b class="c_class_name">child_text</b>
    /// parent_text
    /// </b>
    pub fn find_parent_text(
        doc: &Document, 
        p_tag_name: &str, 
        c_class_name: &str) -> Vec<String> {
        let mut data = vec![];
        for node in doc.find(Name(p_tag_name)) {
            for n in node.children() {
                for (_, cls_name) in n.attrs() {
                    if cls_name == c_class_name {
                        data.push(node.text());
                    } 
                }
            }
        }
        data
    }
    /// it collects all texts by a class name.
    /// <a href="XX" class=class_name title="XXXX">text</a>
    /// parent_text
    /// </b>
    pub fn find_all_text_by_class<'a>(
        doc: &'a Document, 
        class_name: &'a str) -> Vec<String> {     
        let mut data = vec![];
        for node in doc.find(Class(class_name)) {
            data.push(node.text());     
        }
        data
    }
    /// it collects a child attribute by a child class name.
    /// <td>
    /// <a class="class_name" href="XXXX" onclick=XXXX>
    /// <span class="fa fa-cloud-download"></span>
    /// </a>
    /// <a class="class_name" href="XXXX onclick="XXXX rel="XXXX">
    /// <span class="fa fa-magnet"></span>
    /// </a>
    /// </td>
    pub fn find_child_attr_by_class(
        doc: &Document, 
        p_tag_name: &str,
        c_class_name: &str, 
        c_attr_name: &str) -> Vec<String> {
        let mut data = vec![];
        for node in doc.find(Name(p_tag_name)) {
            for n in node.children() {
                for (_, cls_name) in n.attrs() {
                    if cls_name == c_class_name {
                        data.push(n.attr(c_attr_name).unwrap().to_owned());
                    } 
                }
            }
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_magnet_for_torrentsir() {
        let bbs_doc = Document::from(include_str!("./test_data/torrentsir_bbs.html"));
        let data = scrape::find_child_attr_by_tag(
            &bbs_doc, "list-group", "a", "href")[1];
        assert_eq!(
            "magnet:?xt=urn:btih:27646d3df274ed51b6386bd6aa40da849a73b341",
            data,
        );
    }
    #[test]
    fn test_get_get_title_for_torrentsir() {
        let search_doc = Document::from(include_str!("./test_data/torrentsir_search.html"));
        let data = scrape::find_parent_text(&search_doc, "b", "sch_word");
        assert_eq!(
            "동상이몽2 너는 내운명.E182.210201.720p-NEXT",
            &data[0],
        )
    }
    #[test]
    fn test_get_get_bbs_url_for_torrentsir() {
        let search_doc = Document::from(include_str!("./test_data/torrentsir_search.html"));
        let data = scrape::find_child_attr_by_tag(
            &search_doc, "media-heading", "a", "href")[0];
        assert_eq!(
            "./board.php?bo_table=entertain&wr_id=18170",
            data,
        );
    }
    #[test]
    fn test_get_magnet_for_torrentj() {
        let bbs_doc = Document::from(include_str!("./test_data/torrentj_bbs.html"));
        let data = scrape::find_child_attr_by_tag(
            &bbs_doc, "list-group", "a", "href")[1];
        assert_eq!(
            "magnet:?xt=urn:btih:27646d3df274ed51b6386bd6aa40da849a73b341",
            data,
        );
    }
    #[test]
    fn test_get_title_name_for_torrentj() {
        let search_doc = Document::from(include_str!("./test_data/torrentj_search.html"));
        let data = scrape::find_parent_text(&search_doc, "b", "sch_word");
        assert_eq!(
            "동상이몽2 너는 내운명.E182.210201.720p-NEXT",
            &data[0],
        )
    }
    #[test]
    fn test_get_get_bbs_url_for_torrentj() {
        let search_doc = Document::from(include_str!("./test_data/torrentj_search.html"));
        let data = scrape::find_child_attr_by_tag(
            &search_doc, "media-heading", "a", "href")[0];
        assert_eq!(
            "./board.php?bo_table=entertain&wr_id=18172",
            data,
        );
    }
    #[test]
    fn test_get_title_for_torrentview() {
        let search_doc = Document::from(include_str!("./test_data/torrentview_search.html"));
        let data = scrape::find_parent_text(&search_doc, "b", "sch_word");
        assert_eq!(
            "동상이몽2 너는 내운명.E182.210201.720p-NEXT",
            &data[0],
        )
    }
    #[test]
    fn test_get_magnet_for_torrentview() {
        let bbs_doc = Document::from(include_str!("./test_data/torrentview_bbs.html"));
        let data = scrape::find_child_attr_by_tag(
            &bbs_doc, "list-group", "a", "href")[1];
        assert_eq!(
            "magnet:?xt=urn:btih:27646d3df274ed51b6386bd6aa40da849a73b341",
            data,
        );
    }
    #[test]
    fn test_get_get_bbs_url_for_torrentview() {
        let search_doc = Document::from(include_str!("./test_data/torrentview_search.html"));
        let data = scrape::find_child_attr_by_tag(
            &search_doc, "media-heading", "a", "href")[0];
        assert_eq!(
            "./board.php?bo_table=enter&wr_id=21971",
            data,
        );
    }
    #[test]
    fn test_get_title_name_for_tshare() {
        let search_doc = Document::from(include_str!("./test_data/tshare_search.html"));
        let data = scrape::find_parent_text(&search_doc, "p", "sch_word");
        assert_eq!(
            " 아는사이.2020.720p.HDRip-mov18  아는사이.2020.720p.HDRip-mov18",
            &data[0],
        )
    }
    #[test]
    fn test_get_magnet_for_tshare() {
        let bbs_doc = Document::from(include_str!("./test_data/tshare_bbs.html"));
        let data = scrape::find_child_attr_by_tag(
            &bbs_doc, "board-view-torrent-info", "a", "href")[0];
        assert_eq!(
            "magnet:?xt=urn:btih:77c904927c0067cb3aadedae461e20c08eb11164",
            data,
        );
    }
    #[test]
    fn test_get_get_bbs_url_for_tshare() {
        let search_doc = Document::from(include_str!("./test_data/tshare_search.html"));
        let data = scrape::find_child_attr_by_tag(
            &search_doc, "list-item-row", "a", "href")[0];
        assert_eq!(
            "https://tshare.org/movie/11565",
            data,
        );
    }
    #[test]
    fn test_get_title_for_torrentmobile() {
        let search_doc = Document::from(include_str!("./test_data/torrentmobile_search.html"));
        let data = scrape::find_parent_text(&search_doc, "b", "sch_word");
        assert_eq!(
            "동상이몽2 너는 내운명.E182.210201.720p-NEXT",
            &data[0],
        )
    }
    #[test]
    fn test_get_magnet_torrentmobile() {
        let bbs_doc = Document::from(include_str!("./test_data/torrentmobile_bbs.html"));
        let data = scrape::find_child_attr_by_tag(
            &bbs_doc, "list-group", "a", "href")[0];
        assert_eq!(
            "magnet:?xt=urn:btih:27646d3df274ed51b6386bd6aa40da849a73b341",
            data,
        );
    }
    #[test]
    fn test_get_get_bbs_url_for_torrentmobile() {
        let search_doc = Document::from(include_str!("./test_data/torrentmobile_search.html"));
        let data = scrape::find_child_attr_by_tag(
            &search_doc, "media-heading", "a", "href")[0];
        assert_eq!(
            "./board.php?bo_table=music&wr_id=50564",
            data,
        );
    }
    #[test]
    fn test_get_title_for_ttobogo() {
        let search_doc = Document::from(include_str!("./test_data/ttobogo_search.html"));
        let data = &scrape::find_all_text_by_class(&search_doc, "subject")[0];
        assert_eq!(
            "동상이몽2 너는 내운명.E182.210201.720p-NEXT",
            data,
        )
    }
    #[test]
    fn test_get_magnet_ttobogo() {
        let bbs_doc = Document::from(include_str!("./test_data/ttobogo_bbs.html"));
        let data = &scrape::find_child_attr_by_class(
            &bbs_doc, "td", "btn btn-blue", "onclick")[0];
        assert_eq!(
            "file_download(\'magnet:?xt=urn:btih:27646d3df274ed51b6386bd6aa40da849a73b341\')",
            data,
        );
    }
    #[test]
    fn test_get_get_bbs_url_for_ttobogo() {
        let search_doc = Document::from(include_str!("./test_data/ttobogo_search.html"));
        let data = &scrape::find_child_attr_by_class(
            &search_doc, "div", "subject", "href")[0];
        assert_eq!(
            "https://www1.ttobogo.net/post/192852",
            data,
        );
    }
    #[test]
    fn test_get_title_for_torrentsee() {
        let search_doc = Document::from(include_str!("./test_data/torrentsee_search.html"));
        let data = scrape::find_all_text_by_class(&search_doc, "tit");
        assert_eq!(
            "동상이몽2너는내운명.E138.200323.720p-NEXT",
            data[1].trim(),
        )
    }
    #[test]
    fn test_get_magnet_torrentsee() {
        let bbs_doc = Document::from(include_str!("./test_data/torrentsee_bbs.html"));
        let data = &scrape::find_parent_text(
            &bbs_doc, "td", "bbs_btn2")[1];
        assert_eq!(
            "magnet:?xt=urn:btih:eee4d6fdf36ba112523cc48315ac5300cd84c77f",
            data.trim(),
        );
    }
    #[test]
    fn test_get_get_bbs_url_for_torrentsee() {
        let search_doc = Document::from(include_str!("./test_data/torrentsee_search.html"));
        let data = scrape::find_child_attr_by_tag(&search_doc, "tit", "a", "href")[0];
        assert_eq!(
            "/topic/106593",
            data,
        );
    }
}
