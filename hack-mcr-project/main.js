const WEBSTER_API_KEY = '79c9e581-8088-4ce8-9bbb-fa5417a4ed0f';
const YOUTUBE_API_KEY = 'AIzaSyDyzPnArm38HRDDlPcE0p65eCwdZOq8FPE'

async function getWordInfo(word) {
    let response = await fetch(`https://www.dictionaryapi.com/api/v3/references/sd3/json/${word}?key=${WEBSTER_API_KEY.toLowerCase()}`,
        // mode: "no-cors"
    );
    // let jsonString = await response.text();
    // console.log(jsonString);
    // let data = JSON.stringify(jsonString);
    let data = await response.json();
    // console.log(data);
    // data = JSON.stringify(data);
    if (data == null || data == undefined) {
        console.log("The end-of-input error?");
        return null;
    }
    console.log("request thing");
    let result = [];
    console.log(data);

    let definition;
    let phonetics;
    try {
        // for (let i=0; i<data.length; i++) {
        //     console.log(data[i]);
        //     definition = data[i]['shortdef'][0]
        if (data[0].fl == "abbreviation") {
            definition = data[0].shortdef[0];
            phonetics = "";
        } else {
            phonetics = data[0]['hwi']['prs'][0]['mw']
            definition = data[0].shortdef[0];
        }
        //     result.push({word,definition,phonetics});
        // }
        // for (let i=0; i<data[0]['shortdef'].length; i++) {
        //     definition = data[i].shortdef[i];
        //     phonetics = data[i].shortdef[i];
        //     result.push({word,definition,phonetics});
        // }

        if (word == null || word == undefined || definition == null || definition == undefined || phonetics == null || phonetics == undefined) return null;
        return {word, definition, phonetics};
    } catch {
        console.log("error with word " + word);
        return null;
    }
}   

let definitions, promises;
let promisesDone = 0;
const uploadDefinitions = () => {
    console.log(promisesDone, promises.length);
    if (promisesDone == promises.length) {
        document.getElementById("dictionary-data").innerHTML = JSON.stringify(definitions);
        console.log("yaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaay");
    } else requestAnimationFrame(uploadDefinitions);
}

// TODO: make a function which takes an array of words
// and returns an array of objects (each object is the
// return value of getWordInfo)
// and stringify the array
// and put it into the document with id "dictionary-data"

async function getVideoCaptions(videoID) {
    const accessToken = '18a5a88f47f3483cd44c88a70a2a747e';
    const videoId = '877547033';

    const apiUrl = `https://api.vimeo.com/videos/${videoId}/texttracks`;
    const headers = {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    };
    let hlsLink;

    let response = await fetch(apiUrl, {
      method: 'GET',
      headers: headers,
    });

    let data;
    if (response.ok) {
      data = await response.json();
    } else {
      throw new Error('Error loading captions for ' + videoId);
    }
    // Handle the video information in the data variable
    hlsLink = data['data'][0]['hls_link']
    
    data = await (await fetch(hlsLink)).text();
    document.getElementById("captions").innerHTML = data;

    definitions = [];
    let thing = Array.from(data.match(/[a-zA-Z]+/ig));
    console.log(thing);
    let noDuplicates = [];
    for (let i=0; i<thing.length; i++) {
        if (!noDuplicates.includes(thing[i])) noDuplicates.push(thing[i]);
    }
    promises = noDuplicates.map((x, i) => {
        setTimeout(async () => {
            console.log("getting a thing for " + x);
            const definition = await getWordInfo(x.toLowerCase());
            promisesDone += 1;
            if (definition != null && definition != undefined) {
                definitions.push(definition);
                console.log("Defined " + x);
            }
            else console.log("Failed to define " + x);
        }, 150*i);
    });
    console.log("loaded");
    requestAnimationFrame(uploadDefinitions);
    // reject duplicates
    // reject common words

}
let testID = 'DxL2HoqLbyA';
getVideoCaptions(testID).then(data=> console.log(data))