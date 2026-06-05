import { useEffect, useRef, useState } from 'react'
import init, { World } from '../wasm/sim_core.js'

interface Props {
  width: number
  height: number
  count: number
}

export default function SimCanvas({ width, height, count }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const worldRef  = useRef<World | null>(null)      // holds the Rust World instance
  const rafRef    = useRef<number>(0)               // holds the requestAnimationFrame id
  const [stats, setStats] = useState('Loading...')

  useEffect(() => {
    let tickCount = 0
    let lastTime  = performance.now()

    async function start() {
      // Initialise the WASM module — must happen before any Rust calls
      await init()

      worldRef.current = new World(count, width, height)

      const canvas = canvasRef.current!
      const ctx    = canvas.getContext('2d')!

      function loop() {
        const world = worldRef.current!
        world.tick()
        tickCount++

        const positions = world.get_positions()
        const states    = world.get_states()

        ctx.clearRect(0, 0, width, height)

        for (let i = 0; i < world.count(); i++) {
          const x          = positions[i * 2]
          const y          = positions[i * 2 + 1]
          const brightness = 255

          ctx.fillStyle = `rgb(${brightness}, ${brightness}, ${brightness})`
          ctx.fillRect(x - 1, y - 1, 3, 3)
        }

        // Update stats once per second via React state
        const now = performance.now()
        if (now - lastTime >= 1000) {
          setStats(`Automata: ${world.count().toLocaleString()} | Ticks/sec: ${tickCount}`)
          lastTime  = now
          tickCount = 0
        }

        rafRef.current = requestAnimationFrame(loop)
      }

      loop()
    }

    start()

    // Cleanup — cancel the animation loop when the component unmounts
    return () => {
      cancelAnimationFrame(rafRef.current)
    }
  }, [width, height, count]) // re-run if these props change

  return (
    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', padding: '20px' }}>
      <h1 style={{ fontSize: '1.2rem', marginBottom: '10px' }}>Automata Simulation</h1>
      <canvas
        ref={canvasRef}
        width={width}
        height={height}
        style={{ border: '1px solid #444', background: '#000' }}
      />
      <div style={{ marginTop: '10px', fontSize: '0.85rem', color: '#aaa' }}>
        {stats}
      </div>
    </div>
  )
}