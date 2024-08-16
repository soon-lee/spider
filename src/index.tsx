import { render } from 'solid-js/web';
import App from './App';
import GlobalProvider from '@contexts/GlobalProvider';

const root = document.getElementById('root');
if (root) {
  render(() => 
    <GlobalProvider>
      <App />
    </GlobalProvider>, root);
}
