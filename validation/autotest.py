import argparse
import smtplib
import os
try:
    import yaml  # pip install pyyaml
except ModuleNotFoundError:
    print("Please install pyyaml, e.g. with: pip install pyyaml")
    exit(1)
import ssl
import imaplib
import sys


def test_smtp(server: dict):
    """Test if connecting to an SMTP server works.

    :param server: the server dict, part of the provider dict
    """
    host = server["hostname"]
    port = server["port"]
    if server["socket"] == "SSL":
        context = ssl.create_default_context()
        context.set_alpn_protocols(["smtp"])
        smtplib.SMTP_SSL(host, port, context=context)
    elif server["socket"] == "STARTTLS":
        context = ssl.create_default_context()
        context.set_alpn_protocols(["smtp"])
        smtpconn = smtplib.SMTP(host, port)
        smtpconn.starttls(context=context)
        smtpconn.ehlo()
    elif server["socket"] == "PLAIN":
        smtplib.SMTP(host, port)


def test_imap(server: dict):
    """Test if connecting to an IMAP server works.

    :param server: the server dict, part of the provider dict
    :param server: the server dict, part of the provider dict
    """
    host = server["hostname"]
    port = server["port"]
    if server["socket"] == "SSL":
        context = ssl.create_default_context()
        context.set_alpn_protocols(["imap"])
        imaplib.IMAP4_SSL(host, port=port, ssl_context=context)
    elif server["socket"] == "STARTTLS":
        imapconn = imaplib.IMAP4(host, port=port)
        context = ssl.create_default_context()
        context.set_alpn_protocols(["imap"])
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
    parser.add_argument("--path", type=str, default="_providers", help="Path to provider-db/_providers")
    parser.add_argument("--name", type=str, default="", help="Test only a specific provider")
    parser.add_argument("-q", "--quiet", action="store_true", help="Only print errors")
    args = parser.parse_args()

    filenames = get_filenames(args.path)
    providers = [parse_provider(filename) for filename in filenames]

    exitcode = 0
    for provider in providers:
        if provider.get("server") is None:
            continue
        if provider.get("skip_auto_test"):
            continue
        if args.name.lower() not in provider.get("name").lower():
            continue
        for server in provider["server"]:
            if server["hostname"].endswith(".onion"):
                continue  # :todo SOCKS5 support https://gist.github.com/sstevan/efccf3d5d3e73039c21aa848353ff52f
            try:
                if not args.quiet:
                    print("testing %s:%s" % (server["hostname"], server["port"]), end="... ")
                    sys.stdout.flush()
                if server["type"] == "smtp":
                    test_smtp(server)
                if server["type"] == "imap":
                    test_imap(server)
                if not args.quiet:
                    print("done")
            except Exception:
                if args.quiet:
                    print("testing %s:%s" % (server["hostname"], server["port"]), end="... ")
                print("[error] %s: %s" %
                      (sys.exc_info()[0].__name__, sys.exc_info()[1]))
                if "[SSL: DH_KEY_TOO_SMALL]" in str(sys.exc_info()[1]) and provider["name"] == "Naver Mail":
                    continue  # exception: as of 2021-06 naver.com worked; the misconfiguration doesn't seem to be fatal
                if args.name.lower() in provider.get("name").lower() and args.name != "":
                    raise
                exitcode += 1
    return exitcode


if __name__ == "__main__":
    exitcode = main()
    exit(exitcode)
