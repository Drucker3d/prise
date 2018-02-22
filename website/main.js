// thanks to 
//      - https://www.html5rocks.com/en/tutorials/file/dndfiles/
//      - https://www.accelebrate.com/blog/file-uploads-web-sockets-part-3-of-3/
//      - and a few stackoverflow answers
window.onload = function () {
	//var websocket = new WebSocket("ws://127.0.0.1:3012");
	var files = undefined;
	function handleFileSelect(evt) {
		files = evt.target.files; // FileList object
		// files is a FileList of File objects. List some properties.
		var output = [];
		for (var i = 0, file; file = files[i]; i++) {
			output.push('<li><strong>', escape(file.name), '</strong> (',
				file.type || 'n/a', ') - ',
				file.size, ' bytes, last modified: ',
				file.lastModified ? new Date(file.lastModified) : 'n/a',
				'</li>');
		}
		document.getElementById('list').innerHTML = '<ul>' + output.join('') + '</ul>';
		document.getElementById('upload').classList.remove("hidden");
	}


	function uploadFiles() {
		var formData = new FormData();
		for (let i = 0; i < files.length; i++) {
			formData.append("file", files[i]);
		}

		var xhr = new XMLHttpRequest();
		xhr.open("POST", "upload", true);
		xhr.setRequestHeader('X-Requested-With','XMLHttpRequest');
		xhr.upload.onprogress = function(e) {
			if (e.lengthComputable) {
				var percentComplete = (e.loaded / e.total) * 100;
				console.log(percentComplete + '% uploaded');
			}
		};
		xhr.onreadystatechange = function () {
			alert(xhr.readyState + " " + xhr.status);
			if (xhr.readyState == 4 && xhr.status == 200) {
				files = undefined;
				document.getElementById('list').innerHTML = '';
				document.getElementById('upload').classList.add("hidden");
			}
		};

		xhr.send(formData);
	}

	// Check for the various File API support.
	if (window.File && window.FileReader && window.FileList && window.Blob) {
		// Great success! All the File APIs are supported.
		document.getElementById('files').addEventListener('change', handleFileSelect, false);
		document.getElementById('upload').addEventListener('click', uploadFiles, false);
	} else {
		alert('The File APIs are not fully supported in this browser.');
	}
}
