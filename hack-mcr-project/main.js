const WEBSTER_API_KEY = '79c9e581-8088-4ce8-9bbb-fa5417a4ed0f';
const YOUTUBE_API_KEY = 'AIzaSyDyzPnArm38HRDDlPcE0p65eCwdZOq8FPE'

async function getWordInfo(word) {
    let response = await fetch(`https://www.dictionaryapi.com/api/v3/references/sd3/json/${word}?key=${WEBSTER_API_KEY.toLowerCase()}`);
    let data = await response.json();
    let result = [];

    let definition;
    let phonetics;
    // for (let i=0; i<data.length; i++) {
    //     console.log(data[i]);
    //     definition = data[i]['shortdef'][0]
    phonetics = data[0]['hwi']['prs'][0]['mw']
    //     result.push({word,definition,phonetics});
    // }
    // for (let i=0; i<data[0]['shortdef'].length; i++) {
    //     definition = data[i].shortdef[i];
    //     phonetics = data[i].shortdef[i];
    //     result.push({word,definition,phonetics});
    // }
    definition = data[0].shortdef[0];

    return {word, definition, phonetics};
}   

let definitions = [];
let promises = "The quick brown fox jumps over the lazy dog".split(" ").map(x => {
    return getWordInfo(x).then(x => {
        //console.log("defining a thing: " + x);
        definitions.push(x);
    })
});
window.addEventListener('load', () => {
    console.log("loaded");
    requestAnimationFrame(uploadDefinitions);
});

const uploadDefinitions = () => {
    if (definitions.length == promises.length) {
        document.getElementById("dictionary-data").innerHTML = JSON.stringify(definitions);
    } else requestAnimationFrame(uploadDefinitions);
}

// TODO: make a function which takes an array of words
// and returns an array of objects (each object is the
// return value of getWordInfo)
// and stringify the array
// and put it into the document with id "dictionary-data"

async function getVideoCaptions(videoID){
    const captionResourceResponse = await fetch(`https://youtube.googleapis.com/youtube/v3/captions?part=snippet&videoId=${videoID}&key=${YOUTUBE_API_KEY}`);
    let captionResourceData = await captionResourceResponse.json();
    let captionResourceID = captionResourceData['items'][0]['id']

    const captionTrackResponse = await fetch(`https://youtube.googleapis.com/youtube/v3/captions/${captionResourceID}?key=${YOUTUBE_API_KEY}`)
    
    return captionTrackResponse;
}
let testID = 'DxL2HoqLbyA';
getVideoCaptions(testID).then(data=> console.log(data))