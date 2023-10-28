const webster_api_key = '79c9e581-8088-4ce8-9bbb-fa5417a4ed0f'

async function getWordInfo(word) {
    let definition;
    let phonetics;
    let response = await fetch(`https://www.dictionaryapi.com/api/v3/references/sd3/json/${word}?key=${webster_api_key.toLowerCase()}`);
    let data = await response.json();
    definition = data[0]['shortdef'][0]
    phonetics = data[0]['hwi']['prs'][0]['mw']
    return {word,definition,phonetics}  
}   

// TODO: make a function which takes an array of words
// and returns an array of objects (each object is the
// return value of getWordInfo)
// and stringify the array
// and put it into the document with id "dictionary-data"
