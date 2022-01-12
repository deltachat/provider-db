import argparse
import smtplib
import os
import yaml  # pip install pyyaml
import ssl
import imaplib


def test_smtp(server: dict):
    """Test if connecting to an SMTP server works.

    :param server: the server dict, part of the provider dict
    """
    host = server["hostname"]
    port = server["port"]
    print("testing %s:%s" % (host, port))
    if server["socket"] == "SSL":
        smtplib.SMTP_SSL(host, port)
    elif server["socket"] == "STARTTLS":
        smtpconn = smtplib.SMTP(host, port)
        context = ssl.create_default_context()
        smtpconn.starttls(context=context)
        smtpconn.ehlo()
    elif server["socket"] == "PLAIN":
        smtplib.SMTP(host, port)


def test_imap(server: dict):
    """Test if connecting to an IMAP server works.

    :param server: the server dict, part of the provider dict
    """
    host = server["hostname"]
    port = server["port"]
    print("testing %s:%s" % (host, port))
    if server["socket"] == "SSL":
        imaplib.IMAP4_SSL(host, port=port)
    elif server["socket"] == "STARTTLS":
        imapconn = imaplib.IMAP4(host, port=port)
        context = ssl.create_default_context()
        imapconn.starttls(ssl_context=context)
    elif server["socket"] == "PLAIN":
        imaplib.IMAP4(host, port)


def get_filenames(providerspath: str) -> list:
    """Get a list of filenames with provider data

    :param providerspath: the path to the _providers folder
    :return the list of filenames of providers
    """
    path = os.path.join(os.environ["PWD"], providerspath)
    filenames = list()
    for root, dir, files in os.walk(path):
        for filename in files:
            filenames.append(os.path.join(path, filename))
    return filenames


def parse_provider(filename: str) -> dict:
    """Parse a provider file

    :param filename of the provider
    :return the dictionary with values of the provider
    """
    with open(filename) as f:
        raw = f.read()
        providerdata = raw.split("---")[1]
    yml = yaml.load(providerdata, Loader=yaml.SafeLoader)
    yml["freetext"] = raw.split("---")[2]
    return yml


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("path", type=str, default="_providers", help="Path to provider-db/_providers")
    args = parser.parse_args()

    filenames = get_filenames(args.path)
    providers = [parse_provider(filename) for filename in filenames]

    for provider in providers:
        if provider.get("server") is None:
            continue
        for server in provider["server"]:
            if server["type"] == "smtp":
                test_smtp(server)
            if server["type"] == "imap":
                test_imap(server)


if __name__ == "__main__":
    main()
