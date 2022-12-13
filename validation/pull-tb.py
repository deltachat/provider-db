import os
import pathlib
import xml.dom.minidom

import yaml

try:
    from bs4 import BeautifulSoup
except ModuleNotFoundError:
    print("Please install BeautifulSoup, e.g. with: pip install beautifulsoup4")
    exit(1)
import argparse
import requests
from xml.dom import minidom
from pprint import pprint


def get_xml_from_file(ppath: str) -> minidom.parseString:
    """Get provider XML from a file

    :param ppath: path to the provider XML file
    :return: an XML DOM from a certain provider
    """
    with open(ppath, "r") as f:
        content = f.read()
    return minidom.parseString(content)


def get_xml_from_url(url: str) -> minidom.parseString:
    """Download provider XML from a URL

    :param url: URL to a specific provider XML
    :return: an XML DOM from a certain provider
    """
    res = requests.get(url).content
    # print(str(res))
    return minidom.parseString(res)


def get_provider(url: str) -> dict:
    """Get the data of one provider.

    :param url: the URL to the autoconfig XML of the provider.
    :return: the data parsed from the autoconfig XML.
    """
    provider = {}
    if url.startswith("http"):
        parser = get_xml_from_url(url)
    else:
        parser = get_xml_from_file(url)
    prov_element = parser.getElementsByTagName("emailProvider")[0]
    provider["name"] = prov_element.getAttribute("id")
    domains = [domain.firstChild.data for domain in prov_element.getElementsByTagName("domain")]
    provider["domains"] = domains
    # missing: status. not included in TB autoconfig
    for incoming_server in prov_element.getElementsByTagName("incomingServer"):
        if incoming_server.getAttribute("type") == "imap":
            provider["server"] = [
                get_server_data(prov_element.getElementsByTagName("outgoingServer")[0]),
                get_server_data(incoming_server)
            ]
            break
    # some providers have documentation on how to enable IMAP
    provider["status"] = "OK"
    if prov_element.getElementsByTagName("documentation"):
        doc_element = prov_element.getElementsByTagName("documentation")[0]
        provider["status"] = "PREPARATION"
        for descr in doc_element.getElementsByTagName("descr"):
            provider["markdown"] = descr.firstChild.data + ": " + doc_element.getAttribute("url")
            if descr.getAttribute("lang") == "en":
                break  # if english exists, use that
    return provider


def get_server_data(server_element: xml.dom.minidom.Element) -> dict:
    """Get server configuration from an XML element.

    :param server_element: the incomingServer or outgoingServer element
    :return: the dictionary with the server's data.
    """
    server_dict = {
        "type": server_element.getAttribute("type")
    }
    for element in server_element.childNodes:
        if element.localName == "socketType":
            server_dict["socket"] = element.firstChild.data
        elif element.localName == "hostname":
            server_dict["hostname"] = element.firstChild.data
        elif element.localName == "port":
            server_dict["port"] = element.firstChild.data
        elif element.localName == "username":
            server_dict["username"] = element.firstChild.data.strip("%")
    return server_dict


def get_yaml_from_provider(provider: dict) -> str:
    """Format the provider data in YAML so we can write it to a file.

    :param provider: the data parsed from the autoconfig XML.
    :return: a string with YAML data which we can write to a provider-db.md file.
    """
    lines = ["---"]
    lines.append("\nname: " + provider.get("name"))
    lines.append("\nstatus: " + provider.get("status"))
    lines.append("\ndomains:\n")
    lines.append(yaml.dump(provider.get("domains")))
    if yaml.dump(provider.get("server")) != "null\n...\n":
        lines.append("server:\n")
        lines.append(yaml.dump(provider.get("server")))
    lines.append("---\n\n")
    if provider.get("markdown"):
        lines.append(provider.get("markdown"))
    try:
        return ''.join(lines)
    except:
        print(lines)
        raise


def write_yaml_to_file(provider: dict, yaml: str, providers_path: str) -> str:
    """Write the provider YAML to a .md file in the provider path

    :param provider: the data parsed from the autoconfig XML.
    :param yaml: a string with YAML data which we can write to a provider-db.md file.
    :param providers_path: path to provider-db/_providers
    :return path of the file to which the provider data was written.
    """


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--providers-path", "-p", type=str, default="_providers",
                        help="Path to provider-db/_providers")
    parser.add_argument("--root-url", type=str, help="URL or path to XML sources")
    parser.add_argument("--xml-path", type=str, default="tb-autoconfig/ispdb/",
                        help="path to local XML sources")
    parser.add_argument("--dry-run", "-d", action="store_true",
                        help="don't write the TB autoconfig info to the provider-db for now")
    parser.add_argument("--provider-domain", "-u", type=str, help="a single provider to pull")
    parser.add_argument("--just-print-domains", action="store_true", help="just print all domains")
    # parser.add_argument("-q", "--quiet", action="store_true", help="Only print errors")

    args = parser.parse_args()

    provider_urls = set()
    if not args.root_url and pathlib.Path.exists(pathlib.Path(args.xml_path)):
        if args.provider_domain:
            provider_urls.add(args.xml_path + args.provider_domain)
        else:
            for dirpath, _, filenames in os.walk(args.xml_path):
                for filename in filenames:
                    print(dirpath, filename)
                    provider_urls.add(dirpath + filename)
    else:
        root_url = args.root_url
        if root_url is None:
            root_url = "https://autoconfig.thunderbird.net/v1.1/"
        if args.provider_domain:
            provider_urls = set(root_url + args.provider_domain)
        else:
            root_html = requests.get(root_url).content
            root = BeautifulSoup(root_html, features="html.parser")
            for a in root.find_all("a"):
                print(root_url, a["href"])
                provider_urls.add(root_url + a["href"])

    for url in provider_urls:
        if "=" in url or "1.1//" in url:
            continue
        provider = get_provider(url)
        if args.just_print_domains:
            [print(domain) for domain in provider.get("domains")]
            continue
        provider_yaml = get_yaml_from_provider(provider)
        if args.dry_run:
            filename = write_yaml_to_file(provider, provider_yaml, args.providers_path)
            print(provider_yaml)
            continue
        print("written provider info to file:", filename)


if __name__ == "__main__":
    main()
