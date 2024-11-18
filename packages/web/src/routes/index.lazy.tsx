import { createLazyFileRoute } from '@tanstack/react-router';
import { LandingPages } from '@self/modules';

export const Route = createLazyFileRoute('/')({
  component: LandingPages.WelcomePage,
});
