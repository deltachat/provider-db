const fs = require('fs-extra')
const yaml = require('js-yaml')
const { join } = require('path');

const fontmatterRegex = /---\n([^]*\n)---([^]*)/m;

function testServer(server){
    if(!server) throw new Error("Empty servers are not allowed")

    if(!server.hostname || server.hostname.trim() == "") throw new Error("Hostname missing")
    if(!server.port) throw new Error("Port missing")
    if(
        !server.socket ||
        server.socket.trim() == "" ||
        !["SSL", "STARTTLS", "PLAIN"].includes(server.socket.trim())
    ) throw new Error(`Invalid Socket "${server.socket}"`)
    if(
        !server.type ||
        server.type.trim() == "" ||
        !["IMAP", "SMTP"].includes(server.type.trim().toUpperCase())
    ) throw new Error(`Invalid type "${server.type}"`)
    if(
        server.username_pattern &&
        !["EMAIL", "EMAILLOCALPART"].includes(
            server.username_pattern.trim().toUpperCase()
        )
    ) throw new Error(`Invalid username_pattern "${server.username_pattern}"`)

    for (const key of Object.keys(server)) {
        if (![
            "type",
            "socket",
            "hostname",
            "port",
            "username_pattern",
        ].includes(key)) {
            throw new Error(`Unexpected key "${key}"`)
        }
    }
}

function test(fileContent) {
    if (!fontmatterRegex.test(fileContent)) {
        console.log(fileContent);
        throw new Error("Fontmatter not found / bad formatted / not ended (make sure the EOL is set to LF not to CRLF)")
    }
    
    const parseResult = fileContent.match(fontmatterRegex)
    const yamlString = parseResult[1]
    const markdown = parseResult[2]

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
    } else if (json.status != "OK" && json.status != "BROKEN" && json.status != "PREPARATION") {
	// If status != OK, something is wrong. It must be one of these three
	throw new Error("Status is neither OK, BROKEN, nor PREPARATION")
    }

    // is server data populated?
    if(json.server){
        json.server.forEach(server => {
            try {
                testServer(server)
            } catch (error) {
                throw new Error("Error in server definition:" + error.message)
            }
        });
    }

    // Check that config contains only valid keys
    for (const key in json) {
        if (![
            'after_login_hint',
            'before_login_hint',
            'config_defaults',
            'domains',
            'last_checked',
            'name',
            'oauth2',
            'opt',
            'server',
            'status',
            'skip_auto_test',
            'website',
        ].includes(key)) {
            throw new Error(`Unexpected key "${key}"`)
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


