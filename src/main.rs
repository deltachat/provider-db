#[allow(dead_code)]
#[derive(Debug)]
pub enum StatusState {
    OK,
    PREPARATION,
    BROKEN,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Status {
    state: StatusState,
    date: &'static str,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Provider {
    overview_page: &'static str, // for providers.delta.chat/{overview_page}
    name: &'static str,
    status: Status,
    markdown: &'static str,
}

#[allow(dead_code)]
#[derive(Debug)]
struct DomainDBEntry {
    domain: &'static str,
    list_index: u32,
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

fn main() {
    println!("{:?}", get_provider_info("mailbox.org"));

}


pub fn get_provider_info( domain:&str ) -> Option<(&Provider, Vec<&'static str>)> {
    let domain_search_res:Option<&DomainDBEntry> = DOMAIN_DB.iter().find(|e| e.domain == domain);
    if domain_search_res.is_some() {
    let provider_id:u32 = domain_search_res.unwrap().list_index;
      return Some(
        (&DATABASE[provider_id as usize], get_domains_by_provider(provider_id))
      )  
    } else {
        return None
    }
       
}

fn get_domains_by_provider(provider_id:u32) -> Vec<&'static str> {
    return DOMAIN_DB.iter()
        .filter(|entry| entry.list_index == provider_id)
        .map(|e| e.domain).collect();
}

/*
[ ] (exported) get domain name from valid email address
[X] (exported) get provider and its domains from domain name
[X] find all domains with provider id
*/
