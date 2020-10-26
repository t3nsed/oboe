// this is a simple client-side XHR request builder that updates comments 
//in a thread every 5secs

let lastComment = getLastComment();
connect();

function connect() {

    let request = new XMLHttpRequest();
    request.open('GET', window.location.href + "/" + lastComment);

    request.onload = function () {
        if(request.status >= 200 && request.status < 400) {

            let data = JSON.parse(request.responseText);
            console.log(data);
            findComment(data);
        }
        else {
            console.log("Connection with JSON file error!");
        }
    };
    request.send();

    setTimeout(connect, 5000);
}

function getLastComment() {

    if(document.body.childNodes[1].nodeType === 8) {
        let lastComment = document.body.childNodes[1].data;
        console.log(lastComment);
        return lastComment;
    }
    else {
        console.log("Error no Comment");
    }
}

function findComment(comments) {

    for(let i = 0; i < comments.length; i++) {

        let user = comments[i].poster;
        let postId = comments[i].postid;
        let time = comments[i].time;
        let date = comments[i].date;
        let content = comments[i].body;
        let img = comments[i].img;

        if(lastComment < postId) {
            createComment(user, postId, time, date, content, img);
            lastComment = postId;
        }
        //console.log(lastComment);
    }
}

//createComment("1", "2", "3", "4", "5", "6");

function createComment(user, postId, time, date, content, img) {

    let divComment = document.createElement("div");
    divComment.className = "comment";
    document.getElementById("commentSection").appendChild(divComment);

    let divInfo = document.createElement("div");
    divComment.appendChild(divInfo);

    let divContent = document.createElement("div");
    divContent.className = "content";
    divComment.appendChild(divContent);

    let image = document.createElement("img");
    image.className = "imgThread";
    image.alt = "image not found";
    image.src = "/" + img;

    let pContent = document.createElement("p");
    pContent.innerHTML = content;

    divContent.appendChild(image);
    divContent.appendChild(pContent);

    let divUser = document.createElement("div");
    let divId = document.createElement("div");
    let divTime =  document.createElement("div");
    let divDate = document.createElement("div");

    divUser.className = "user";
    divId.className = "id";
    divTime.className = "time";
    divDate.className = "date";

    divUser.innerHTML = user;
    divId.innerHTML = "ID: " + postId;
    divTime.innerHTML = time;
    divDate.innerHTML = date;

    divInfo.appendChild(divUser);
    divInfo.appendChild(divId);
    divInfo.appendChild(divTime);
    divInfo.appendChild(divDate);
}