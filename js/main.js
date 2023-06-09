import init, { draw } from '../pkg/ray_tracing_wasm.js';

const console_output = document.getElementById('console');
const console_error = console.error;
console.error = function(...objs){
	console_output.value = objs.join("\n");
	console_error(...objs);
}

await init();
  
const canvas = document.getElementById('drawing');
const render_btn = document.getElementById('render');
const scene_input = document.getElementById("scene-json");
const text_input = document.getElementById("json");
const img_width_input = document.getElementById("img-width");
const img_height_input = document.getElementById("img-height");
const samples_per_pixel_input = document.getElementById("samples-per-pixel");
const max_bounces_input = document.getElementById("max-bounces");

scene_input.addEventListener("change", async function(e){
	const scene_json = await (await fetch(e.target.value)).json();
	text_input.value = JSON.stringify(scene_json, "null", "\t");
	
	render_btn.click();
});

render_btn.addEventListener('click', () => {
	if(!img_width_input.reportValidity() || Number.isNaN(img_width_input.valueAsNumber)) throw new Error("Image width is invalid!");
	if(!img_height_input.reportValidity() || Number.isNaN(img_height_input.valueAsNumber)) throw new Error("Image height is invalid!");
	if(!samples_per_pixel_input.reportValidity() || Number.isNaN(samples_per_pixel_input.valueAsNumber)) throw new Error("Samples Per Pixel is invalid!");
	if(!max_bounces_input.reportValidity() || Number.isNaN(max_bounces_input.valueAsNumber)) throw new Error("Max Reflection Bounces is invalid!");
	
	canvas.width = img_width_input.valueAsNumber;
	canvas.height = img_height_input.valueAsNumber;
	
	const ctx = canvas.getContext('2d');
	draw(ctx, text_input.value, img_width_input.valueAsNumber, img_height_input.valueAsNumber, samples_per_pixel_input.valueAsNumber, max_bounces_input.valueAsNumber);
});

const scene_json = await (await fetch("scenes/materials.json")).json();
text_input.value = JSON.stringify(scene_json, "null", "\t");

render_btn.click();