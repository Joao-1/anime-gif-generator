### Anime Gif Generator

The Anime GIF Generator is a tool that analyses animations and creates GIFs from transitions between scenes. The AGG analyses every frame of the provided animation and compares the last and current frames to determine if a transition has occurred. If a transition is detected, a new GIF is created using all frames that make up that scene.

<div>
  <img  src="https://github.com/Joao-1/anime-gif-generator/assets/58475277/ce5d85ff-52f6-4c74-86b3-423faf4df511"  />
</div>

This is the Rust version of my old [Python](https://github.com/Joao-1/gifGenerator) code without too many differences, but it does have some issues listed below:

### Issues
#### Time
The process of opening a video, analysing the frames to find for a transition and creating a GIF works almost perfectly (I'll talk about other problems later). However, processing the frame data takes ~9 seconds at high quality and ~1 second at low quality. This is terrible and takes a lot of time to create a small number of GIFs. To illustrate this, I set up the application to create GIFs from episode 1 of "K-on!!" and it took about 12 hours to go through the whole video and create all the GIFs of the episode. 

I don't know why this happens in Rust but not in Python. Maybe the crate I'm using isn't optimised, or my logic needs to be different in Rust.

#### False positive
Another issue identified is the high number of false positives when detecting scene transitions, particularly on anime with action scenes. The analysis system may generate a false positive if there are spells, high velocity, or excessive lighting present. This is because it analyses changes in pixels and provides a percentage of the difference. If the percentage exceeds a default value, the system will identify it as a new transition, even if it is not.

The number of false positives is low for slice of life animes such as K-on, Non Non Biyori, and Yure Camp. However, animes with many action scenes, such as Kimetsu No Yaiba, Sousou No Frieren, and Jujutsu Kaisen, have many false positive GIFs.

#### FPS
The most awkward and annoying issue is the low FPS of the GIFs, which makes them unusable.  It is possible that OpenCV is the cause, but I am unsure how to improve this. See the example below:
![gif1706690361](https://github.com/Joao-1/anime-gif-generator/assets/58475277/917254c5-f163-41b1-80a9-7f11142d15f5)
Simply unusable.

### Tasks
- [ ] Add tests
- [ ] Compress generated GIFs to reduce their file size. Some GIFs can be over 100Mb, making them unusable.
- [ ] Make it useable as a CLI tool
