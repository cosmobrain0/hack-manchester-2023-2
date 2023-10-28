const webster_api_key = '79c9e581-8088-4ce8-9bbb-fa5417a4ed0f';
const youtube_api_key = 'AIzaSyDyzPnArm38HRDDlPcE0p65eCwdZOq8FPE';

async function getWordInfo(word) {
    let definition;
    let phonetics;
    let response = await fetch(`https://www.dictionaryapi.com/api/v3/references/sd3/json/${word}?key=${webster_api_key.toLowerCase()}`);
    let data = await response.json();
    definition = data[0]['shortdef'][0];
    phonetics = data[0]['hwi']['prs'][0]['mw'];
    return [definition,phonetics]  
}   
async function getClosedCaptions(videoId){
    let response = await fetch(`https://youtube.googleapis.com/youtube/v3/captions?part=snippet&videoId=${videoId}&key=${youtube_api_key}`)
    let data = await response.json();
    let captionTrackID = data['items'][0]['id']
   
    const res2 = await fetch(`https://youtube.googleapis.com/youtube/v3/captions/${captionTrackID}`,{
        method: 'GET',
        authorization: 'Bearer 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',
        accept: 'application/json'
        
    })
    let captions = await res2.json();

    
    return captions;
}
 getClosedCaptions('tEjQxvgiDs4').then(data=>{console.log(data)});