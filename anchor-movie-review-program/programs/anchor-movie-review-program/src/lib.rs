use anchor_lang::prelude::*;

declare_id!("FwQJSBuiLhxJFQh16t2CS8ekNFrqSWbfsrqoH4kUmqVZ");

#[program]
pub mod anchor_movie_review_program {
    use super::*;
    pub fn add_movie_review(ctx: Context<AddMovieReview>, title:String, description:String, rating:u8)->Result<()>{
        print!("Moview Review Account created");
        print!("Title: {}", title);
        print!("Description: {}", description);
        print!("Rating: {}", rating);
        
        let movie_review= &mut ctx.accounts.movie_review;
        movie_review.title=title;
        movie_review.description=description;
        movie_review.rating=rating;
        movie_review.reviewer=ctx.accounts.initializer.key();
        Ok(())
    }

    pub fn update_movie_review(ctx: Context<UpdateMovieReview>, title:String, description:String, rating:u8)->Result<()>{
        print!("Moview Review Account space reallocated");
        print!("Title: {}", title);
        print!("Description: {}", description);
        print!("Rating: {}", rating);
        
        let movie_review= &mut ctx.accounts.movie_review;
        movie_review.rating=rating;
        movie_review.description=description;
        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title:String)->Result<()>{
        print!("Moview Review for {} deleted", title);
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(title:String, description:String)]
pub struct AddMovieReview<'info>{
    #[account(
        init,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer= initializer,
        space= 8 + 32 + 1 + 4 + title.len() + 4 + description.len()
    )]
    pub movie_review:Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String, description:String)]
pub struct UpdateMovieReview<'info>{
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc= 8 + 32 + 1 + 4 + title.len() + 4 + description.len(),
        realloc::payer=initializer,
        realloc::zero=true,
    )]
    pub movie_review:Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteMovieReview<'info>{
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub movie_review:Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[account]
pub struct MovieAccountState{
    pub reviewer: Pubkey, //32
    pub rating: u8, //1
    pub title: String, //4 + len()
    pub description: String, //4 + len()
}
