window.onload = function() {
    document.getElementById('btn_install').addEventListener('click', function() {
        document.getElementById('progress_bar').classList.add('active');
        window.external.invoke('install');
    })
};

function downloadCompleted() {
    document.getElementById('progress_bar').classList.remove('active');
};
