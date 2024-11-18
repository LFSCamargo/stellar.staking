import { useWelcomeQuery } from '../../queries';
import WelcomeTemplate from './template';

const WelcomePage = () => {
  const { data } = useWelcomeQuery();

  return <WelcomeTemplate name={data?.name || ''} />;
};

export default WelcomePage;
