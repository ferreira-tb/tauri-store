import '../../.shared/assets/style.css';
import App from '@/App';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

const root = document.querySelector('#root')!;

createRoot(root).render(
  <StrictMode>
    <App />
  </StrictMode>
);
