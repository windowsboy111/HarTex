import { MetaTags } from '@redwoodjs/web'

const DocumentationPage = () => {
  return (
    <>
      <MetaTags title="Documentation" description="HarTex" />

      <div className="flex overflow-hidden">
        <div className="flex-[0_0_350px] overflow-hidden bg-dark-grey">
        </div>
        <div className="flex-[1_1_1440px] overflow-hidden">
        </div>
      </div>
    </>
  )
}

export default DocumentationPage
