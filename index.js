import { start, Application } from './pkg';

start();

const canvas = document.createElement('canvas');
canvas.width = 1080;
canvas.height = 720;
document.body.appendChild(canvas);

const app = new Application(canvas);

const loop = () => {
	app.update();
	requestAnimationFrame(loop);
}
requestAnimationFrame(loop);