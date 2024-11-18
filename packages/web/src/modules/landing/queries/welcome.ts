import { useQuery } from '@tanstack/react-query';

export function useWelcomeQuery() {
  return useQuery<{
    name: string;
  }>({
    queryKey: ['welcome'],
    queryFn: async () => {
      return new Promise((resolve) =>
        setTimeout(
          () =>
            resolve({
              name: 'John Doe',
            }),
          1000,
        ),
      );
    },
  });
}
