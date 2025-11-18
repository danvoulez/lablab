<template>
  <div class="chart-container">
    <div ref="chartRef" class="chart"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import * as d3 from 'd3'

interface DataPoint {
  timestamp: string | Date
  value: number
}

interface Props {
  data: DataPoint[]
  height?: number
  color?: string
  showGrid?: boolean
  animate?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  height: 300,
  color: '#00d4ff',
  showGrid: true,
  animate: true
})

const chartRef = ref<HTMLDivElement | null>(null)

const renderChart = () => {
  if (!chartRef.value || !props.data.length) return

  // Clear previous chart
  d3.select(chartRef.value).selectAll('*').remove()

  const container = chartRef.value
  const width = container.clientWidth
  const height = props.height

  const margin = { top: 20, right: 30, bottom: 40, left: 60 }
  const innerWidth = width - margin.left - margin.right
  const innerHeight = height - margin.top - margin.bottom

  // Create SVG
  const svg = d3.select(chartRef.value)
    .append('svg')
    .attr('width', width)
    .attr('height', height)
    .style('background', 'transparent')

  const g = svg.append('g')
    .attr('transform', `translate(${margin.left},${margin.top})`)

  // Parse data
  const parseTime = d3.timeParse('%Y-%m-%dT%H:%M:%S.%LZ')
  const data = props.data.map(d => ({
    timestamp: typeof d.timestamp === 'string' ? parseTime(d.timestamp) : d.timestamp,
    value: d.value
  })).filter(d => d.timestamp !== null) as Array<{ timestamp: Date; value: number }>

  // Scales
  const x = d3.scaleTime()
    .domain(d3.extent(data, d => d.timestamp) as [Date, Date])
    .range([0, innerWidth])

  const y = d3.scaleLinear()
    .domain([0, d3.max(data, d => d.value) || 100])
    .nice()
    .range([innerHeight, 0])

  // Grid lines
  if (props.showGrid) {
    g.append('g')
      .attr('class', 'grid')
      .attr('opacity', 0.1)
      .call(d3.axisLeft(y)
        .tickSize(-innerWidth)
        .tickFormat(() => '')
      )
  }

  // Gradient
  const gradient = svg.append('defs')
    .append('linearGradient')
    .attr('id', 'line-gradient')
    .attr('gradientUnits', 'userSpaceOnUse')
    .attr('x1', 0).attr('y1', y(0))
    .attr('x2', 0).attr('y2', y(d3.max(data, d => d.value) || 100))

  gradient.append('stop')
    .attr('offset', '0%')
    .attr('stop-color', props.color)
    .attr('stop-opacity', 1)

  gradient.append('stop')
    .attr('offset', '100%')
    .attr('stop-color', props.color)
    .attr('stop-opacity', 0.3)

  // Area gradient
  const areaGradient = svg.append('defs')
    .append('linearGradient')
    .attr('id', 'area-gradient')
    .attr('gradientUnits', 'userSpaceOnUse')
    .attr('x1', 0).attr('y1', y(0))
    .attr('x2', 0).attr('y2', y(d3.max(data, d => d.value) || 100))

  areaGradient.append('stop')
    .attr('offset', '0%')
    .attr('stop-color', props.color)
    .attr('stop-opacity', 0.4)

  areaGradient.append('stop')
    .attr('offset', '100%')
    .attr('stop-color', props.color)
    .attr('stop-opacity', 0.05)

  // Line generator
  const line = d3.line<{ timestamp: Date; value: number }>()
    .x(d => x(d.timestamp))
    .y(d => y(d.value))
    .curve(d3.curveMonotoneX)

  // Area generator
  const area = d3.area<{ timestamp: Date; value: number }>()
    .x(d => x(d.timestamp))
    .y0(innerHeight)
    .y1(d => y(d.value))
    .curve(d3.curveMonotoneX)

  // Draw area
  const areaPath = g.append('path')
    .datum(data)
    .attr('fill', 'url(#area-gradient)')
    .attr('d', area)

  // Draw line
  const linePath = g.append('path')
    .datum(data)
    .attr('fill', 'none')
    .attr('stroke', 'url(#line-gradient)')
    .attr('stroke-width', 2)
    .attr('d', line)
    .style('filter', `drop-shadow(0 0 8px ${props.color}80)`)

  // Animation
  if (props.animate) {
    const totalLength = linePath.node()?.getTotalLength() || 0
    linePath
      .attr('stroke-dasharray', `${totalLength} ${totalLength}`)
      .attr('stroke-dashoffset', totalLength)
      .transition()
      .duration(1500)
      .ease(d3.easeQuadInOut)
      .attr('stroke-dashoffset', 0)
  }

  // Axes
  const xAxis = g.append('g')
    .attr('transform', `translate(0,${innerHeight})`)
    .call(d3.axisBottom(x).ticks(6))
    .style('color', 'rgba(255, 255, 255, 0.5)')

  xAxis.select('.domain').style('stroke', 'rgba(255, 255, 255, 0.1)')
  xAxis.selectAll('.tick line').style('stroke', 'rgba(255, 255, 255, 0.1)')

  const yAxis = g.append('g')
    .call(d3.axisLeft(y).ticks(5))
    .style('color', 'rgba(255, 255, 255, 0.5)')

  yAxis.select('.domain').style('stroke', 'rgba(255, 255, 255, 0.1)')
  yAxis.selectAll('.tick line').style('stroke', 'rgba(255, 255, 255, 0.1)')

  // Dots
  g.selectAll('.dot')
    .data(data)
    .enter()
    .append('circle')
    .attr('class', 'dot')
    .attr('cx', d => x(d.timestamp))
    .attr('cy', d => y(d.value))
    .attr('r', 0)
    .attr('fill', props.color)
    .style('filter', `drop-shadow(0 0 4px ${props.color})`)
    .transition()
    .delay((d, i) => props.animate ? i * 50 : 0)
    .duration(300)
    .attr('r', 4)
}

onMounted(() => {
  renderChart()
})

watch(() => [props.data, props.height, props.color], () => {
  renderChart()
}, { deep: true })
</script>

<style scoped>
.chart-container {
  width: 100%;
  position: relative;
}

.chart {
  width: 100%;
}

:deep(.grid line) {
  stroke: rgba(255, 255, 255, 0.05);
}

:deep(.grid path) {
  stroke-width: 0;
}
</style>
