import { createRoot } from 'react-dom/client'
import SimCanvas from './components/SimCanvas'

createRoot(document.getElementById('root')!).render(
  <SimCanvas width={800} height={600} count={1000} />
)