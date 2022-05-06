try:
    from bs4 import BeautifulSoup
except ModuleNotFoundError:
    print("Please install BeautifulSoup, e.g. with: pip install beautifulsoup4")
    exit(1)
import argparse
import requests
from xml.dom import minidom


def get_provider(url: str) -> dict:
    """Get the data of one provider.

    :param url: the URL to the autoconfig XML of the provider.
    :return: the data parsed from the autoconfig XML.
    """
    provider = {}
    res = requests.get(url).content
    #print(str(res))
    parser = minidom.parseString(res)
    prov_element = parser.getElementsByTagName("emailProvider")[0]
    provider["name"] = prov_element.getAttribute("id")
    domains = [domain.firstChild.data for domain in prov_element.getElementsByTagName("domain")]
    provider["domains"] = domains
    return provider


def get_yaml_from_provider(provider: dict) -> str:
    """Format the provider data in YAML so we can write it to a file.

    :param provider: the data parsed from the autoconfig XML.
    :return: a string with YAML data which we can write to a provider-db.md file.
    """
    return str(provider)


def write_yaml_to_file(provider: dict, yaml: str, providers_path: str) -> str:
    """Write the provider YAML to a .md file in the provider path

    :param provider: the data parsed from the autoconfig XML.
    :param yaml: a string with YAML data which we can write to a provider-db.md file.
    :param providers_path: path to provider-db/_providers
    :return path of the file to which the provider data was written.
    """


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--providers_path", "-p", type=str, default="_providers",
                        help="Path to provider-db/_providers")
    parser.add_argument("--root-url", type=str, default="https://autoconfig.thunderbird.net/v1.1/",
                        help="URL to XML sources")
    parser.add_argument("--dry-run", "-d", action="store_true",
                        help="don't write the TB autoconfig info to the provider-db for now")
    parser.add_argument("--provider-url", "-u", type=str, help="a single provider you want to pull")
    # parser.add_argument("-q", "--quiet", action="store_true", help="Only print errors")

    args = parser.parse_args()

    provider_urls = set()
    if args.provider_url:
        provider_urls.add(args.root_url + args.provider_url)
    else:
        root_html = requests.get(args.root_url).content
        root = BeautifulSoup(root_html, features="html.parser")
        for a in root.find_all("a"):
            provider_urls.add(args.root_url + a["href"])

    for url in provider_urls:
        provider = get_provider(url)
        provider_yaml = get_yaml_from_provider(provider)
        if args.dry_run:
            print("YAML:", provider_yaml)
            continue
        filename = write_yaml_to_file(provider, provider_yaml, args.providers_path)
        print("written provider info to file:", filename)


if __name__ == "__main__":
    main()