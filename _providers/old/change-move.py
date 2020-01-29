#!/usr/bin/env python3

import yaml
import xml.etree.ElementTree as ET
from re import sub
from os.path import exists
from os import remove


def get_yaml(file):
    with open(file + ".md", "r", encoding="UTF-8") as f:
        raw = f.read()
        raw = raw[4:]
        raw = sub(r'---(?s).*', '', raw)
        raw_yaml = yaml.safe_load(raw)
    return raw_yaml


def get_xml(raw_yml):
    i = 0
    file = ""
    if raw_yml['domains'] is dict:
        while not exists(file):
            file = "/home/user/Desktop/autoconfig.thunderbird.net/" + raw_yml['domains'][i]
            i += 1
    elif type(raw_yml['domains']) is str:
        file = "/home/user/Desktop/autoconfig.thunderbird.net/" + raw_yml['domains']
    return ET.parse(file).getroot()


def get_servers_from_xml(raw_yml):
    # add server data from XML
    try:
        raw_xml = get_xml(raw_yml)
    except IndexError:
        return
    servers = []
    for child in raw_xml:
        for child2 in child:
            if child2.tag == "incomingServer" or child2.tag == "outgoingServer":
                if child2.attrib['type'] == "imap" or child2.attrib['type'] == "smtp":
                    server = {"type": child2.attrib['type']}
                    for child3 in child2:
                        if child3.tag == "hostname":
                            server["hostname"] = child3.text
                        if child3.tag == "port":
                            server["port"] = child3.text
                        if child3.tag == "socketType":
                            server["socket"] = child3.text
                        if child3.tag == "username" and child3.text == "%EMAILLOCALPART":
                            server["username"] = "EMAILLOCALPART"
                    servers.append(server)
    return servers


files = ["all-inkl.com",
         "bluewin.ch",
         "comcast",
         "dismail.de",
         "disroot",
         "freenet.de",
         "hosteurope",
         "i.ua",
         "kolst.com",
         "kontent.com",
         "mailbox.org",
         "mail.ru",
         "manitu.de",
         "nauta.cu",
         "posteo",
         "riseup.net",
         "rogers.com",
         "strato.de",
         "tiscali.it",
         "ukr.net",
         "verizon",
         "vfemail",
         "yandex.ru",
         "ziggo.nl",
         "zoho.com", ]

for file in files:
    # get YML from file.md
    raw_yml = get_yaml(file)

    # initalize dictionary, add values from yaml
    new_yml = {"name": raw_yml["name"],
               "status": "OK",
               "domains": raw_yml["domains"],
               # get XML from TB autoconfig
               }

    if "date" in raw_yml["status"]:
        new_yml["last_checked"] = raw_yml["status"]["date"]
    if "website" in raw_yml:
        new_yml["website"] = raw_yml["website"]
    try:
        new_yml["server"]: get_servers_from_xml(raw_yml)
    except FileNotFoundError:
        pass

        # write to ../file.md
    with open("../" + file + ".md", "a+") as output:
        output.write("---\n")
        yaml.dump(new_yml, output, default_flow_style=False)
        output.write("---\n")
        #print("---")
        #print(yaml.safe_dump(new_yml, default_flow_style=False), end="")
        #print("---")

    # remove old file
    remove(file + ".md")
