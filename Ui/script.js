// Song playlist with online audio sources
const songs = [
    {
        title: "Chill Vibes",
        artist: "Lofi Artist",
        url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3",
        videoId: "song_1"
    },
    {
        title: "Summer Breeze",
        artist: "Relaxing Sounds",
        url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-2.mp3",
        videoId: "song_2"
    },
    {
        title: "Night Dreams",
        artist: "Sleep Music",
        url: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-3.mp3",
        videoId: "song_3"
    }
];

let currentSongIndex = 0;
let isPlaying = false;

const audioPlayer = document.getElementById('audio-player');
const songTitle = document.getElementById('song-title');
const songArtist = document.getElementById('song-artist');
const playPauseBtn = document.getElementById('play-pause-btn');
const prevBtn = document.getElementById('prev-btn');
const nextBtn = document.getElementById('next-btn');
const progress = document.getElementById('progress');
const currentTimeEl = document.getElementById('current-time');
const durationEl = document.getElementById('duration');
const playlistItems = document.getElementById('playlist-items');
const progressBar = document.querySelector('.progress-bar');

// Initialize playlist
function initPlaylist() {
    songs.forEach((song, index) => {
        const li = document.createElement('li');
        li.innerHTML = `
            <div class="song-item-title">${song.title}</div>
            <div class="song-item-artist">${song.artist}</div>
        `;
        li.addEventListener('click', () => loadSong(index));
        playlistItems.appendChild(li);
    });
}

// Load song
function loadSong(index) {
    currentSongIndex = index;
    const song = songs[index];
    
    audioPlayer.src = song.url;
    songTitle.textContent = song.title;
    songArtist.textContent = song.artist;
    
    // Update active state in playlist
    document.querySelectorAll('#playlist-items li').forEach((li, i) => {
        li.classList.toggle('active', i === index);
    });
    
    // Update Discord Rich Presence via Tauri
    updateDiscordPresence(song);
}

// Play/Pause functionality
function togglePlayPause() {
    if (isPlaying) {
        audioPlayer.pause();
        playPauseBtn.textContent = '▶️';
        notifyDiscordPause();
    } else {
        audioPlayer.play();
        playPauseBtn.textContent = '⏸';
        notifyDiscordPlay();
    }
    isPlaying = !isPlaying;
}

// Previous song
function prevSong() {
    currentSongIndex = (currentSongIndex - 1 + songs.length) % songs.length;
    loadSong(currentSongIndex);
    if (isPlaying) {
        audioPlayer.play();
    }
}

// Next song
function nextSong() {
    currentSongIndex = (currentSongIndex + 1) % songs.length;
    loadSong(currentSongIndex);
    if (isPlaying) {
        audioPlayer.play();
    }
}

// Update progress bar
function updateProgress() {
    const { duration, currentTime } = audioPlayer;
    if (duration) {
        const progressPercent = (currentTime / duration) * 100;
        progress.style.width = `${progressPercent}%`;
        
        currentTimeEl.textContent = formatTime(currentTime);
        durationEl.textContent = formatTime(duration);
        
        // Update Discord with current position
        updateDiscordPosition(currentTime);
    }
}

// Format time
function formatTime(time) {
    if (isNaN(time)) return '0:00';
    const minutes = Math.floor(time / 60);
    const seconds = Math.floor(time % 60);
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}

// Seek functionality
function seek(e) {
    const width = progressBar.clientWidth;
    const clickX = e.offsetX;
    const duration = audioPlayer.duration;
    
    audioPlayer.currentTime = (clickX / width) * duration;
}

// Discord Rich Presence integration via Tauri
async function updateDiscordPresence(song) {
    if (window.__TAURI__) {
        try {
            await window.__TAURI__.invoke('update_discord_presence', {
                title: song.title,
                artist: song.artist,
                videoId: song.videoId,
                duration: audioPlayer.duration || 0
            });
        } catch (error) {
            console.log('Discord update failed:', error);
        }
    }
}

async function updateDiscordPosition(position) {
    if (window.__TAURI__) {
        try {
            await window.__TAURI__.invoke('update_discord_position', {
                position: position
            });
        } catch (error) {
            console.log('Discord position update failed:', error);
        }
    }
}

async function notifyDiscordPlay() {
    if (window.__TAURI__) {
        try {
            await window.__TAURI__.invoke('discord_set_playing', {
                playing: true
            });
        } catch (error) {
            console.log('Discord play notification failed:', error);
        }
    }
}

async function notifyDiscordPause() {
    if (window.__TAURI__) {
        try {
            await window.__TAURI__.invoke('discord_set_playing', {
                playing: false
            });
        } catch (error) {
            console.log('Discord pause notification failed:', error);
        }
    }
}

// Event listeners
playPauseBtn.addEventListener('click', togglePlayPause);
prevBtn.addEventListener('click', prevSong);
nextBtn.addEventListener('click', nextSong);
audioPlayer.addEventListener('timeupdate', updateProgress);
audioPlayer.addEventListener('ended', nextSong);
audioPlayer.addEventListener('loadedmetadata', () => {
    durationEl.textContent = formatTime(audioPlayer.duration);
    if (window.__TAURI__) {
        updateDiscordPresence(songs[currentSongIndex]);
    }
});
progressBar.addEventListener('click', seek);

// Initialize
initPlaylist();
loadSong(0);
