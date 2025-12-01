import { onMounted } from 'vue'

export function useAudio() {
    let audioCtx: AudioContext | null = null

    onMounted(() => {
        audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)()
    })

    const resumeAudioContext = async () => {
        if (audioCtx && audioCtx.state === 'suspended') {
            await audioCtx.resume()
        }
    }

    const playBeep = () => {
        if (!audioCtx) return
        const osc = audioCtx.createOscillator()
        const gain = audioCtx.createGain()
        osc.type = 'sine'
        osc.frequency.value = 800 + Math.random() * 200
        osc.connect(gain)
        gain.connect(audioCtx.destination)
        osc.start()
        gain.gain.exponentialRampToValueAtTime(0.00001, audioCtx.currentTime + 0.1)
        osc.stop(audioCtx.currentTime + 0.1)
    }

    const playWin = () => {
        if (!audioCtx) return
        const now = audioCtx.currentTime

        // 和音: C Major (C, E, G)
        const notes = [523.25, 659.25, 783.99, 1046.50]

        notes.forEach((freq, i) => {
            const osc = audioCtx!.createOscillator()
            const gain = audioCtx!.createGain()
            osc.type = 'triangle'
            osc.frequency.value = freq
            osc.connect(gain)
            gain.connect(audioCtx!.destination)

            osc.start(now + i * 0.05)
            gain.gain.setValueAtTime(0, now + i * 0.05)
            gain.gain.linearRampToValueAtTime(0.5, now + i * 0.05 + 0.1)
            gain.gain.exponentialRampToValueAtTime(0.001, now + 2.0)
            osc.stop(now + 2.0)
        })
    }

    return {
        resumeAudioContext,
        playBeep,
        playWin,
    }
}
