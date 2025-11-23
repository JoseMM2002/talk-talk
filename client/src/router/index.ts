import { createRouter, createWebHistory } from 'vue-router';

import { DefaultLayout } from '@/layouts';
import { InboxHeader } from '@/layouts/header';
import { InboxView, ServerView } from '@/views';

const ROUTES = {
  INBOX: 'INBOX',
  SERVER: 'SERVER',
} as const;

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: DefaultLayout,
      children: [
        {
          path: 'inbox',
          name: ROUTES.INBOX,
          components: {
            default: InboxView,
            'header-title': InboxHeader,
          },
        },
        {
          path: 'server/:id',
          name: ROUTES.SERVER,
          components: {
            default: ServerView,
          },
        },
        {
          path: '',
          redirect: { name: ROUTES.INBOX },
        },
        {
          path: ':otherRoutes',
          redirect: { name: ROUTES.INBOX },
        },
      ],
    },
  ],
});

export default router;
