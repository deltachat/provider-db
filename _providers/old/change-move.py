#!/usr/bin/env python3

import yaml
import xml.etree.ElementTree as ET
from re import sub
from os.path import exists


def get_yaml(file):
    with open(file + ".md", "r", encoding="UTF-8") as f:
        raw = f.read()
        raw = raw[4:]
        raw = sub(r'---(?s).*', '', raw)
        print(raw)
        raw_yaml = yaml.safe_load(raw)
    return raw_yaml

def get_xml(raw_yml):
    i = 0
    file = ""
    while not exists(file):
        file = "/home/user/Desktop/autoconfig.thunderbird.net/" + raw_yml['domains'][i]
        i += 1
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


files = ["comcast"]

for file in files:
    # get YML from file.md
    raw_yml = get_yaml(file)
    print(raw_yml)

    # initalize dictionary, add values from yaml
    new_yml = {"name": raw_yml["name"],
               "status": "OK",
               "domains": raw_yml["domains"],
               # get XML from TB autoconfig
               "server": get_servers_from_xml(raw_yml),
               }

    if raw_yml["status"]["date"]:
        new_yml["last_checked"] = raw_yml["status"]["date"]
    if raw_yml["website"]:
        new_yml["website"] = raw_yml["website"]

    print(new_yml)

    # write to ../file.md
    with open("../" + file + ".md", "a+") as output:
        output.write("---\n")
        yaml.dump(new_yml, output, default_flow_style = False)
        output.write("---\n")
