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
let promises = "The quick brown fox jumps over the lazy dog".split(" ").map(async x => {
    const definition = await getWordInfo(x);
    definitions.push(definition);
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
    const accessToken = '18a5a88f47f3483cd44c88a70a2a747e';
    const videoId = '877547033';

    const apiUrl = `https://api.vimeo.com/videos/${videoId}/texttracks`;
    const headers = {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    };
    let hlsLink;

    fetch(apiUrl, {
      method: 'GET',
      headers: headers,
    })
      .then(response => {
        if (response.ok) {
          return response.json();
        } else {
          throw new Error('Error');
        }
      })
      .then(data => {
        // Handle the video information in the data variable
        hlsLink = data['data'][0]['hls_link']
        fetch(hlsLink)
        .then((response) => response.text())
        .then((data)=>console.log(data));
    })
      .catch(error => {
        // Handle any errors here
        console.error(error);
      });
      console.log(hlsLink)
}
let testID = 'DxL2HoqLbyA';
getVideoCaptions(testID).then(data=> console.log(data))