export const muscleData = {
  // Simplified body outline (less complex curves)
  outline: 'M200,100 L300,100 L350,200 L350,700 L300,800 L200,800 L150,700 L150,200 Z',
  
  muscles: {
    // Upper Body - More geometric shapes
    shoulders: 'M200,150 L300,150 L290,200 L210,200 Z',
    chest: 'M200,200 L300,200 L300,250 L200,250 Z',
    upper_back: 'M200,200 L150,220 L150,300 L200,320',
    biceps: 'M180,220 L180,280 L200,300 L200,240',
    triceps: 'M320,220 L320,280 L300,300 L300,240',
    
    // Core - Simplified curves
    abs: 'M200,300 Q250,350 300,300 T250,450',
    
    // Lower Body - More defined muscle groups
    quads: 'M200,400 L220,500 L230,600 L250,650 L270,600 L280,500 L300,400 Z',
    hamstrings: 'M200,400 L180,500 L170,600 L175,650 L180,600 L190,500 L210,400',
    glutes: 'M200,350 L220,400 L250,420 L280,400 L300,350',
    calves: 'M200,600 L220,650 L250,670 L280,650 L300,600'
  },
  
  // Optimized label positions
  labels: {
    shoulders: { x: 250, y: 170, fontSize: '0.8em' },
    chest: { x: 250, y: 225, fontSize: '0.8em' },
    upper_back: { x: 170, y: 250, fontSize: '0.7em' },
    biceps: { x: 190, y: 250, fontSize: '0.7em' },
    triceps: { x: 310, y: 250, fontSize: '0.7em' },
    abs: { x: 250, y: 350, fontSize: '0.8em' },
    quads: { x: 250, y: 500, fontSize: '0.8em' },
    hamstrings: { x: 190, y: 500, fontSize: '0.7em' },
    glutes: { x: 250, y: 380, fontSize: '0.8em' },
    calves: { x: 250, y: 630, fontSize: '0.8em' }
  },
  
  // Additional styling parameters
  style: {
    outline: { stroke: '#333', strokeWidth: 2, fill: 'transparent' },
    muscles: { 
      default: { fill: 'rgba(200, 50, 50, 0.3)', stroke: 'rgba(200, 50, 50, 0.8)' },
      highlighted: { fill: 'rgba(255, 80, 80, 0.6)' }
    },
    labels: { 
      fontFamily: 'Arial, sans-serif',
      inactive: { fill: '#666' },
      active: { fill: '#d00', fontWeight: 'bold' }
    }
  }
};