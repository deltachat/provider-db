const fs = require('fs-extra')
const yaml = require('js-yaml')
const { join } = require('path');

const fontmatterRegex = /---\n([^]*\n)---([^]*)/m;

function testServer(server){
    if(!server) throw new Error("Empty servers are not allowed")

    if(!server.hostname || server.hostname.trim() == "") throw new Error("Hostname missing")
    if(!server.port) throw new Error("Port missing")
    if(!server.socket || server.socket.trim() == "") throw new Error("Socket missing")
    if(!server.type || server.type.trim() == "") throw new Error("Type missing")

    // todo username_pattern optional but one of right the values
}

function test(fileContent) {
    if (fontmatterRegex.test(fontmatterRegex)) {
        throw new Error("Fontmatter not found / bad formatted / not ended")
    }
    const yamlString = fileContent.match(fontmatterRegex)[1]
    const markdown = fileContent.match(fontmatterRegex)[2]

    const json = yaml.load(yamlString);

    if (json.status == "PREPARATION") {
        // If status == PREPARATION, does before_login_hint exist?
        if (!json.before_login_hint || json.before_login_hint.trim() === "") {
            throw new Error("Status is PREPARATION, but 'before_login_hint' is missing")
        }
        // If status == PREPARATION, does markdown exist? (maybe even require screenshots?)
        if (markdown.trim() === "") {
            throw new Error("Status is PREPARATION, but website content is missing")
        }
    } else if (json.status == "BROKEN") {
        // If status == BROKEN, does before_login_hint & after_login_hint exist?
        if (!json.before_login_hint || json.before_login_hint.trim() === "") {
            throw new Error("Status is BROKEN, but 'before_login_hint' is missing")
        }
        if (markdown.trim() === "") {
            throw new Error("Status is BROKEN, but website content is missing")
        }
    }

    // is server data populated?
    if(json.server){
        var has_smtp = false;
        var has_imap = false;
        json.server.forEach(server => {
            try {
                testServer(server)
                if (server.type == "imap") {
                    has_imap = true
                } else if (server.type == "smtp") {
                    has_smtp = true
                }
            } catch (error) {
                throw new Error("Error in server definition:" + error.message)
            }
        });
        if(!(has_imap && has_smtp)){
            throw new Error("Server definition needs atlease one server of both types")
        }
    }
}

(async () => {

    const providers = await fs.readdir(join(__dirname, '../_providers'))
    let success = true

    for (let i = 0; i < providers.length; i++) {
        const provider = providers[i];
        const fileContent = await fs.readFile(join(__dirname, '../_providers', provider), 'utf-8')

        try {
            test(fileContent)
        } catch (error) {
            console.log(`Error in ${provider}:\n`, error.name, error.message)
            success = false
        }
    }

    process.exit(success?0:1)
})()


