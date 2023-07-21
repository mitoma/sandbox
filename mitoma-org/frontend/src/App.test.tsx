import { render, screen } from '@testing-library/react';
import App from './App';

test('renders learn react link', () => {
  render(<App />);
  const siteTitle = screen.getByText(/mitoma.org/i);
  expect(siteTitle).toBeInTheDocument();
});
