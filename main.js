const webster_api_key = '79c9e581-8088-4ce8-9bbb-fa5417a4ed0f'

async function getWordInfo(word) {
    let definition;
    let phonetics;
    let response = await fetch(`https://www.dictionaryapi.com/api/v3/references/sd3/json/${word}?key=${webster_api_key.toLowerCase()}`);
    let data = await response.json();
    definition = data[0]['shortdef'][0]
    phonetics = data[0]['hwi']['prs'][0]['mw']
    return [definition,phonetics]  
}   