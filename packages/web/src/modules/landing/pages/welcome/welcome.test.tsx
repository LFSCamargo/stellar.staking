import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';

import WelcomeScreen from '.';

vi.mock('../../queries', () => ({
  useWelcomeQuery: vi.fn().mockReturnValue({ data: { name: 'John Doe' } }),
}));

describe('Tests the welcome page', () => {
  it('should render', () => {
    const { getByText } = render(<WelcomeScreen />);

    expect(getByText('Welcome, John Doe')).toBeTruthy();
    expect(getByText('Life is good, you know what I mean?')).toBeTruthy();
  });
});
