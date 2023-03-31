import { h, Component, render } from 'https://unpkg.com/preact?module';
import htm from 'https://unpkg.com/htm?module';
import "https://unpkg.com/@antv/g2@beta/dist/g2.min.js";

// const html = htm.bind(h);

// function App(props) {
//   return html`
//     <div>
//       id = ${props.id}
//     </div>
//   `;
// }

// render(html`<${App} id="container" />`, document.body);

// 准备数据
const data = [
    { genre: 'Sports', sold: 275 },
    { genre: 'Strategy', sold: 115 },
    { genre: 'Action', sold: 120 },
    { genre: 'Shooter', sold: 350 },
    { genre: 'Other', sold: 150 },
];

// Step 1: 创建 Chart 对象
const chart = new G2.Chart({
    container: 'container', // 指定图表容器 ID
    width: 600, // 指定图表宽度
    height: 300, // 指定图表高度
});


// Step 3：创建图形语法，
chart.interval()
.data(data)
// .position('genre*sold')
// .color('genre');
.encode("x", "genre")
.encode("y", "sold")
.style('fill', '#1890ff')
.style('stroke', '#fff');

// Step 4: 渲染图表
chart.render();


