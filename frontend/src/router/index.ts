import { createRouter, createWebHistory } from 'vue-router'
import BingoView from '../views/BingoView.vue'
import AmidaView from '../views/AmidaView.vue'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'bingo',
            component: BingoView
        },
        {
            path: '/amida',
            name: 'amida',
            component: AmidaView
        },
        {
            path: '/amida/result',
            name: 'amida-result',
            component: AmidaView
        }
    ]
})

export default router
